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
    "description" VARCHAR NOT NULL,
    "stars" INT DEFAULT 0 NOT NULL,
    "downloads" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("plugin_id") REFERENCES "plugins"("id"),
    UNIQUE ("plugin_id", "version")
);

CREATE FUNCTION update_plugins_downloads_stars() 
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
    AS
$$
BEGIN
    UPDATE plugins 
        SET plugins.downloads = plugins.downloads + NEW.downloads - OLD.downloads, plugins.stars = NEW.stars - OLD.stars;
    RETURN NEW;
END;
$$;

CREATE TRIGGER sum_plugins_download_downloads
    AFTER UPDATE ON plugin_versions
    FOR EACH ROW
    EXECUTE PROCEDURE update_plugins_downloads_stars();

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

CREATE TABLE "plugin_version_downloads"(
    "plugin_version_id" INT NOT NULL,
    "arch" VARCHAR NOT NULL,
    "size" INT NOT NULL,
    "downloads" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("plugin_version_id", "arch")
);

CREATE FUNCTION update_plugin_versions_downloads() 
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
    AS
$$
BEGIN
    UPDATE plugin_versions 
        SET plugin_versions.downloads = plugin_versions.downloads + NEW.downloads - OLD.downloads;
    RETURN NEW;
END;
$$;

CREATE TRIGGER sum_plugin_version_download
    AFTER UPDATE ON plugin_versions
    FOR EACH ROW
    EXECUTE PROCEDURE update_plugin_versions_downloads();