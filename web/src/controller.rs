use actix_web::{web, HttpResponse, get};
use crate::AppData;
use crate::error::DictError;
use database::search::PronunciationType;
use database::schema::{word_pronunciation, word, word_entry};
use database::diesel::prelude::*;
use database::diesel::result::Error as DieselError;
use actix_web::error::BlockingError;
use database::models::{Word, WordEntry, WordPronunciation};
use serde::Serialize;
use std::collections::HashMap;
use database::DbConnection;

#[derive(Serialize)]
struct WordResult {
    simplified: String,
    traditional: String,
    entries: HashMap<i32, Vec<EntryResult>>,
}
#[derive(Serialize)]
struct EntryResult {
    definitions: String,
    pronunciations: HashMap<i32, Vec<String>>,
}

#[get("/search/{type:jyutping|pinyin}/{query}")]
pub(crate) async fn pronunciation_search(data: web::Data<AppData>, path: web::Path<(String, String)>) -> Result<HttpResponse, DictError> {
    let query = &*path.1;
    let dict_search = &data.dict_search;
    let conn = data.database_pool.clone();

    let pronunciation_type = match &*path.0 {
        "jyutping" => PronunciationType::Jyutping,
        "pinyin" => PronunciationType::Pinyin,
        _ => unreachable!(),
    };

    let pronunciation_ids = match dict_search.search_pronunciation(query, pronunciation_type) {
        Ok(r) => r,
        Err(e) => return Err(DictError::Search(e)),
    };

    let db_results: Vec<_> = web::block(move || {
        let connection = &conn.get_connection();

        let result: Vec<i32> = word::table.inner_join(word_entry::table)
            .inner_join(word_pronunciation::table.on(
                word_entry::entry_id.eq(word_pronunciation::entry_id)))
            .filter(word_pronunciation::pronunciation_id.eq_any(pronunciation_ids))
            .select(word::word_id)
            .load(connection)?;

        get_word_results(result, connection)
    }).await.map_err(|e| {
        match e {
            BlockingError::Canceled => DictError::Actix,
            BlockingError::Error(e) => DictError::Database(e),
        }
    })?;

    Ok(HttpResponse::Ok().json(db_results))
}


#[get("/search/characters/{query}")]
pub(crate) async fn character_search(data: web::Data<AppData>, path: web::Path<String>) -> Result<HttpResponse, DictError> {
    let query = &*path;
    let dict_search = &data.dict_search;
    let conn = data.database_pool.clone();

    let word_ids = match dict_search.search_characters(query) {
        Ok(r) => r,
        Err(e) => return Err(DictError::Search(e)),
    };

    let db_results: Vec<_> = web::block(move || {
        let connection = &conn.get_connection();

        get_word_results(word_ids, connection)
    }).await.map_err(|e| {
        match e {
            BlockingError::Canceled => DictError::Actix,
            BlockingError::Error(e) => DictError::Database(e),
        }
    })?;

    Ok(HttpResponse::Ok().json(db_results))
}

fn get_word_results(word_ids: Vec<i32>, connection: &DbConnection) -> Result<Vec<WordResult>, DieselError> {
    let words = word::table.filter(word::word_id.eq_any(word_ids))
        .load::<Word>(connection)?;
    let entries = WordEntry::belonging_to(&words).load::<WordEntry>(connection)?;
    let pronunciations = WordPronunciation::belonging_to(&entries).load::<WordPronunciation>(connection)?;

    let pronunciations_grouped = pronunciations.grouped_by(&entries);
    let entries_full = entries.into_iter().zip(pronunciations_grouped).collect::<Vec<_>>().grouped_by(&words);
    let words_full = words.into_iter().zip(entries_full).collect::<Vec<_>>();

    let results: Vec<WordResult> = words_full.iter().map(|x| {
        let mut result = WordResult {
            simplified: x.0.simplified.clone(),
            traditional: x.0.traditional.clone(),
            entries: HashMap::new(),
        };
        for entry in &x.1 {
            let mut entry_result = EntryResult {
                definitions: entry.0.definitions.clone(),
                pronunciations: HashMap::new(),
            };
            for pronunciation in &entry.1 {
                entry_result.pronunciations.entry(pronunciation.pronunciation_type)
                    .or_insert_with(|| Vec::new())
                    .push(pronunciation.pronunciation.clone());
            }

            result.entries.entry(entry.0.dictionary_id)
                .or_insert_with(|| Vec::new())
                .push(entry_result);
        }

        result
    }).collect();

    Ok(results)
}