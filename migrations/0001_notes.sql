create table notes (
    note_id varchar not null primary key,
    title varchar not null,
    content varchar not null,
    done boolean not null default false
);
create unique index note_id_idx on notes (note_id);