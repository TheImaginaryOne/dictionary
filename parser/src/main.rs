mod cedict_parser;
mod types;

use structopt::StructOpt;

use crate::types::Entry;
use database::diesel;
use database::DbConnection;
use database::diesel::connection::Connection; // so we can do transactions
use database::diesel::result::Error as DieselError;
use database::diesel::prelude::*;

#[derive(Debug)]
enum ParserError {
    FileError,
    CedictError,
    DbError(DieselError),
}

#[derive(StructOpt)]
#[structopt(
    name = "dict-parser",
    about = "Parses a modified version of CEDICT and CC-Canto files."
)]
struct Opts {
    #[structopt(short = "i", long)]
    in_file: String,
    #[structopt(short = "d", long)]
    dictionary_id: i32,
}

fn main() {
    match parse() {
        Ok(_) => (),
        Err(e) => println!("Error occurred {:?}", e) // todo
    };
}

fn parse() -> Result<(), ParserError> {
    let opt = Opts::from_args();

    let pool = database::create_db_pool();

    let input = std::fs::read_to_string(opt.in_file).map_err(|_| ParserError::FileError)?;
    println!("Parsing data");
    let mut result = cedict_parser::parse_cedict(&input).map_err(|_| ParserError::CedictError)?;

    for entry in &mut result.1 {
        entry.dictionary_id = opt.dictionary_id;
    }
    insert_entries(&result.1, &pool.get_connection(), opt.dictionary_id)?;

    Ok(())
}

fn insert_entries(entries: &Vec<Entry>, connection: &DbConnection, dict_id: i32) -> Result<(), ParserError> {

    use database::schema::word_entry;
    use diesel::sql_types::{Integer, Text};
    use diesel::connection::SimpleConnection;

    connection.transaction::<_, DieselError, _>(|| {
        println!("Deleting entries");
        diesel::delete(word_entry::table.filter(word_entry::columns::dictionary_id.eq(dict_id))).execute(connection)?;

        println!("Creating temporary table");
        // direct execution, no need for sql_query
        connection.execute(include_str!("scripts/create_temp.sql"))?;
        println!("Inserting data");

        let mut data = Vec::new();
        for entry in entries {
            if entry.jyutping != "" {
                data.push((entry.traditional.clone(), entry.simplified.clone(), dict_id, entry.definition.clone(), 1, entry.jyutping.clone()));
            }
            if entry.pinyin != "" {
                data.push((entry.traditional.clone(), entry.simplified.clone(), dict_id, entry.definition.clone(), 0, entry.pinyin.clone()));
            }
        }

        let insert = "insert into temp_data values (?,?,?,?,?,?)".to_string();
        for record in &data {
            diesel::sql_query(&insert).bind::<Text, _>(&record.0)
                .bind::<Text, _>(&record.1)
                .bind::<Integer, _>(record.2)
                .bind::<Text, _>(&record.3)
                .bind::<Integer, _>(record.4)
                .bind::<Text, _>(&record.5)
                .execute(connection)?;
        }

        println!("Moving data from temporary table to main tables");
        connection.batch_execute(include_str!("scripts/move_temp_data.sql"))?;
        Ok(())
    }).map_err(|e| ParserError::DbError(e))?;

    Ok(())
}