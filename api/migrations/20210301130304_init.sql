CREATE TABLE "users"(
    "id" INT NOT NULL,
    "name" VARCHAR NOT NULL,
    "login" VARCHAR NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE "user_tokens"(
    "id" SERIAL NOT NULL,
    "user_id" INT NOT NULL,
    "name" VARCHAR NOT NULL,
    "secret" BYTEA NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    "used" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users"("id"),
    UNIQUE ("secret"),
    UNIQUE ("user_id", "name")
);

CREATE TABLE "plugins"(
    "id" SERIAL NOT NULL,
    "name" VARCHAR NOT NULL,
    "stars" VARCHAR NOT NULL,
    "downloads" INT DEFAULT 0 NOT NULL,
    "lts_version_id" INT NULL,
    PRIMARY KEY ("id"),
    UNIQUE ("name")
);

CREATE TABLE "plugin_owners"(
    "plugin_id" INT NOT NULL,
    "user_id" INT NOT NULL,
    PRIMARY KEY ("plugin_id", "user_id"),
    FOREIGN KEY ("plugin_id") REFERENCES "plugins"("id"),
    FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE TABLE "plugin_owner_invites"(
    "plugin_id" INT NOT NULL,
    "user_id" INT NOT NULL,
    PRIMARY KEY ("plugin_id", "user_id"),
    FOREIGN KEY ("plugin_id") REFERENCES "plugins"("id"),
    FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE TABLE "plugin_versions"(
    "id" SERIAL NOT NULL,
    "plugin_id" INT NOT NULL,
    "version" VARCHAR NOT NULL,
    "summary" VARCHAR NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    "stars" INT DEFAULT 0 NOT NULL,
    "downloads" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("plugin_id") REFERENCES "plugins"("id"),
    UNIQUE ("plugin_id", "version")
);

CREATE TABLE "plugin_version_categories"(
    "plugin_version_id" INT NOT NULL,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin_version_id", "name"),
    FOREIGN KEY ("plugin_version_id") REFERENCES "plugin_versions"("id")
);

CREATE TABLE "plugin_version_authors"(
    "plugin_version_id" INT NOT NULL,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin_version_id", "name"),
    FOREIGN KEY ("plugin_version_id") REFERENCES "plugin_versions"("id")
);

CREATE TABLE "plugin_version_artifacts"(
    "id" SERIAL NOT NULL,
    "plugin_version_id" INT NOT NULL,
    "arch" VARCHAR NOT NULL,
    "size" INT NOT NULL,
    "downloads" INT DEFAULT 0 NOT NULL,
    "downloads_recent" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("id"),
    UNIQUE ("plugin_version_id", "arch")
);

CREATE TABLE "plugin_version_artifact_daily_downloads"(
    "plugin_version_artifact_id" INT NOT NULL,
    "date" DATE DEFAULT NOW(),
    "downloads" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("plugin_version_artifact_id", "date"),
    FOREIGN KEY ("plugin_version_artifact_id") REFERENCES "plugin_version_artifacts"("id")
);