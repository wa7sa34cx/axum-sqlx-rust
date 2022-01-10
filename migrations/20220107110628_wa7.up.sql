-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS wa7_test_id_seq;

-- Table Definition
CREATE TABLE IF NOT EXISTS "public"."wa7_test" (
    "id" int4 NOT NULL DEFAULT nextval('wa7_test_id_seq'::regclass),
    "text" varchar(128) NOT NULL,
    PRIMARY KEY ("id")
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS wa7_test_null_id_seq;

-- Table Definition
CREATE TABLE IF NOT EXISTS "public"."wa7_test_null" (
    "id" int4 NOT NULL DEFAULT nextval('wa7_test_null_id_seq'::regclass),
    "text" varchar(128),
    PRIMARY KEY ("id")
);