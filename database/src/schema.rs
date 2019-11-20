table! {
    word (word_id) {
        word_id -> Int4,
        traditional -> Text,
        simplified -> Text,
    }
}

table! {
    word_entry (entry_id) {
        entry_id -> Int4,
        word_id -> Int4,
        dictionary_id -> Int4,
        definitions -> Text,
    }
}

table! {
    word_pronunciation (pronunciation_id) {
        pronunciation_id -> Int4,
        pronunciation_type -> Int4,
        pronunciation -> Text,
        entry_id -> Int4,
    }
}

joinable!(word_entry -> word (word_id));
joinable!(word_pronunciation -> word_entry (entry_id));

allow_tables_to_appear_in_same_query!(
    word,
    word_entry,
    word_pronunciation,
);
