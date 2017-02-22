CREATE TABLE company_devices (
  id SERIAL PRIMARY KEY,
  company_id SERIAL NOT NULL,
  name VARCHAR NOT NULL DEFAULT 'Device',
  mac VARCHAR NOT NULL
)
