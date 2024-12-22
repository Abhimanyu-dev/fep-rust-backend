create table if not exists projects(
  id int primary key generated always as identity,
  offered_by text,
  description text not null,
  img text,
  files text,
  FOREIGN KEY(offered_by) REFERENCES professors(email)
);
