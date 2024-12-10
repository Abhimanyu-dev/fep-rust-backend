create table if not exists students(
  email text not null,
  cpi float not null,
  branch text not null,
  batch int not null,
  roll_no int not null,
  FOREIGN KEY(email) REFERENCES users(email)
);
