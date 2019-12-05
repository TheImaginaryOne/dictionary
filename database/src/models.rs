use super::schema::{word, word_entry, word_pronunciation};

#[derive(Queryable)]
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
#[derive(Queryable)]
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

#[derive(Queryable, Debug)]
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