-- Tables!

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  fullname VARCHAR NOT NULL DEFAULT 'User',
  avatar VARCHAR NOT NULL DEFAULT 'default.jpg',
  background VARCHAR NOT NULL DEFAULT 'Flocking_by_noombox.jpg'
);

CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  user_id SERIAL NOT NULL
);

CREATE TABLE companies (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  ip VARCHAR NOT NULL DEFAULT '127.0.0.1',
  user_id SERIAL NOT NULL
);

CREATE TABLE wemos (
  id SERIAL PRIMARY KEY,
  company_id SERIAL NOT NULL,
  local_ip VARCHAR NOT NULL,
  device_1 SERIAL NOT NULL,
  device_2 SERIAL NOT NULL
);

CREATE TABLE devices (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  status BOOLEAN NOT NULL DEFAULT FALSE
);
