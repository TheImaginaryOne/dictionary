use super::schema::{word, word_entry, word_pronunciation};
use serde::Serialize;

#[derive(Serialize)]
#[derive(Queryable, Identifiable)]
#[table_name = "word"]
#[primary_key(word_id)]
pub struct Word {
    pub word_id: i32,
    pub traditional: String,
    pub simplified: String,
}
#[derive(Insertable)]
#[table_name = "word"]
pub struct NewWord {
    pub traditional: String,
    pub simplified: String,
}
#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Word)]
#[table_name = "word_entry"]
#[primary_key(entry_id)]
pub struct WordEntry {
    pub entry_id: i32,
    pub word_id: i32,
    pub dictionary_id: i32,
    pub definitions: String,
}

#[derive(Insertable)]
#[table_name = "word_entry"]
pub struct NewWordEntry {
    pub word_id: i32,
    pub dictionary_id: i32,
    pub definitions: String,
}

#[derive(Queryable, Debug, Associations, Identifiable)]
#[belongs_to(WordEntry, foreign_key = "entry_id")]
#[table_name = "word_pronunciation"]
#[primary_key(pronunciation_id)]
pub struct WordPronunciation {
    pub pronunciation_id: i32,
    pub pronunciation_type: i32,
    pub pronunciation: String,
    pub entry_id: i32,
}

#[derive(Insertable)]
#[table_name = "word_pronunciation"]
pub struct NewWordPronunciation {
    pub pronunciation_type: i32,
    pub pronunciation: String,
    pub entry_id: i32,
}
