create table word(
    word_id serial primary key not null,
    traditional text not null,
    simplified text not null,
    unique(traditional, simplified)
);

create table word_entry(
    entry_id serial primary key not null,
    word_id integer not null,
    dictionary_id integer not null,
    definitions text not null,
    constraint fk_word_entry_word foreign key(word_id) references word(word_id) on update cascade on delete cascade,
    unique(word_id, dictionary_id, definitions)
);

create table word_pronunciation(
    pronunciation_id serial primary key not null,
    pronunciation_type integer not null,
    pronunciation text not null,
    entry_id integer not null,
    constraint fk_word_pronunciation_word_entry foreign key(entry_id) references word_entry(entry_id) on update cascade on delete cascade,
    unique(pronunciation_id, pronunciation_type, entry_id)
);