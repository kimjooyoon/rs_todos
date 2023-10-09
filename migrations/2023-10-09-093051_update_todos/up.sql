-- Your SQL goes here
DROP TABLE todos;

CREATE TABLE todos
(
    id              bigserial PRIMARY KEY,
    title           VARCHAR(50) UNIQUE NOT NULL,
    content         VARCHAR(300) not null ,
    order_number       int not null ,
    is_deleted      bool not null ,
    created_at      timestamptz,
    last_updated_at timestamptz
);
