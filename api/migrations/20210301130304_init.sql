CREATE TABLE "users"(
    "id" INT NOT NULL,
    "name" VARCHAR NOT NULL,
    "login" VARCHAR NOT NULL,
    "email" VARCHAR NULL,
    "created_at" TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE "user_tokens"(
    "id" SERIAL NOT NULL,
    "user_id" INT NOT NULL,
    "name" VARCHAR NULL,
    "secret" BYTEA NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    "used_total" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users"("id"),
    UNIQUE ("name")
);