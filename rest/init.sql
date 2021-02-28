CREATE TABLE "plugins"(
    "id" SERIAL,
    "name" VARCHAR,
    PRIMARY KEY ("id")
);

CREATE TABLE "plugin_owners"(
    "plugin_id" VARCHAR,
    "user_id" INT,
    PRIMARY KEY ("plugin_id", "user_id")
);

CREATE TABLE "users"(
    "id" SERIAL,
    "name" VARCHAR,
    PRIMARY KEY ("id")
);

CREATE TABLE "plugin_versions"(
    "id" SERIAL,
    "plugin_id" INT,
    "version" INT,
    PRIMARY KEY ("id"),
    UNIQUE ("plugin_id", "version")
);