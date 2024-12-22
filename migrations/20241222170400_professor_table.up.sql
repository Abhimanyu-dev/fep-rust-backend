create table if not exists professors(
  email text not null,
  institute text not null,
  PRIMARY KEY(email),
  FOREIGN KEY(email) REFERENCES users(email)
);
