-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
  '513510fa-6d2b-4152-b742-cc6275cce7af',
  'admin',
  '$argon2id$v=19$m=15000,t=2,p=1$2te3fyc9EZJaGpPd4SGbfQ$nuLbZ4pl/D7fT6g/tgUQiBgL2rqE35ZJPt1gn8SKYx0'
);
