table! {
    word (word_id) {
        word_id -> Integer,
        traditional -> Text,
        simplified -> Text,
    }
}

table! {
    word_entry (entry_id) {
        entry_id -> Integer,
        word_id -> Integer,
        dictionary_id -> Integer,
        definitions -> Text,
    }
}

table! {
    word_pronunciation (pronunciation_id) {
        pronunciation_id -> Integer,
        pronunciation_type -> Integer,
        pronunciation -> Text,
        entry_id -> Integer,
    }
}

joinable!(word_entry -> word (word_id));
joinable!(word_pronunciation -> word_entry (entry_id));

allow_tables_to_appear_in_same_query!(
    word,
    word_entry,
    word_pronunciation,
);
