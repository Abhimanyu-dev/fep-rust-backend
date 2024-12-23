create table if not exists applications(
  id int primary key generated always as identity,
  project_id int not null,
  student text not null,
  status text not null default 'pending',
  note text not null,
  file text not null,
  FOREIGN KEY(project_id) REFERENCES projects(id),
  FOREIGN KEY(student) REFERENCES students(email)
);
