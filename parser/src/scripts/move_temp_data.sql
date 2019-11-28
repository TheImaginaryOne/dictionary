insert or ignore into word (traditional, simplified)
select traditional, simplified from temp_data;

insert or ignore into word_entry (word_id, dictionary_id, definitions)
select word_id, dictionary_id, definitions from temp_data
join word
    on word.traditional = temp_data.traditional
    and word.simplified = temp_data.simplified;

-- might be a bit slow
insert or ignore into word_pronunciation (pronunciation_type, pronunciation, entry_id)
select pronunciation_type, pronunciation, entry_id from temp_data
join word_entry we
    on we.definitions = temp_data.definitions
    and we.dictionary_id = temp_data.dictionary_id
join word
    on we.word_id = word.word_id
    and word.traditional = temp_data.traditional
    and word.simplified = temp_data.simplified;