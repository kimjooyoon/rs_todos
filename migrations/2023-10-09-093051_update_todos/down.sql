-- This file should undo anything in `up.sql`
DROP TABLE todos;

CREATE TABLE todos
(
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(50) UNIQUE NOT NULL,
    content         VARCHAR(300),
    order_num       int,
    is_deleted      bool,
    created_at      timestamptz,
    last_updated_at timestamptz
);
