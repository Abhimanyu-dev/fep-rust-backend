create table if not exists users(
  id int generated always as identity,
  name text not null,
  email text not null,
  password text not null,
  role_id int not null,
  primary key(id)
);
