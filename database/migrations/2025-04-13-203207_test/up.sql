-- Your SQL goes here
create table test (
    id serial primary key,
    name text not null,
    created_at timestamp default now(),
    updated_at timestamp default now()
);