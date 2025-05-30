-- SQLNESS REPLACE (\d+:\d+:\d+\.\d+) TIME
-- SQLNESS REPLACE [\s\-]+
select current_time();

select GREATEST('1999-01-30', '2023-03-01');

select GREATEST('2000-02-11'::Date, '2020-12-30'::Date);

select GREATEST('2021-07-01 00:00:00'::Timestamp, '2024-07-01 00:00:00'::Timestamp);
