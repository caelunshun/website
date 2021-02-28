
# Rest
Feather rest api, located at `https://api.feather.rs`

### Plugin publish
```
POST /plugins
```

### Plugins
```
GET /plugins
```
#### Parameters
- start_timestamp
- page?
- search?

```json
[
    {
        "name": "",
        "description": "",
        "homepage": "",
        "repository": "",
        "downloads": 0,
        "recent_downloads": 0,
        "timestamp": 0,
    }
]
```

### Plugin get
```
GET /plugins/:id
```

```json
{
    "owners": ["user_identifier"],
    "versions": ["0.1", "0.5", "1.0"]
}

```

### Plugin get latest version details
```
GET /plugins/:id/latest
```

temporary redirect to the latest version

### Plugin get version details
```
GET /plugins/:id/:version
```

```json
{
    "name": "",
    "keywords": [],
    "categories": [],
    "authors": [],
    "dependencies": [],
    "size": 4096,
    "timestamp": 0,
    "downloads": 0,
    "stars": 0,
}
```

### Plugin yank version
```
DELETE /plugins/:id/:version
```

# DB
```sql
CREATE TABLE "plugins"(
    "id" SERIAL NOT NULL,
    "name" VARCHAR NOT NULL,
    "downloads" INT DEFAULT 0 NOT NULL,
    "stars" INT DEFAULT 0 NOT NULL,
    PRIMARY KEY ("id"),
);

CREATE TABLE "plugin_versions"(
    "plugin" INT NOT NULL,
    "version" VARCHAR NOT NULL,
    "downloads" INT DEFAULT 0 NOT NULL,
    "stars" INT DEFAULT 0 NOT NULL,
    "size" INT NOT NULL,
    PRIMARY KEY ("plugin", "version"),
    FOREIGN KEY ("plugin") REFERENCES "plugins"("id"),
);

CREATE TABLE "plugin_version_downloads"(
    "plugin" INT NOT NULL,
    "plugin_version" VARCHAR NOT NULL,
    "arch" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin", "plugin_version", "arch"),
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("plugin", "version"),
);

CREATE TABLE "plugin_version_keywords"(
    "plugin" INT NOT NULL,
    "plugin_version" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin", "plugin_version", "keyword"),
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("plugin", "version"),
);

CREATE TABLE "plugin_version_categories"(
    "plugin_id" INT NOT NULL,
    "plugin_version" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin", "plugin_version", "keyword"),
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("plugin", "version"),
);

CREATE TABLE "plugin_version_dependencies"(
    "plugin_id" INT NOT NULL,
    "plugin_version" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin", "plugin_version", "name"),
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("id", "version"),
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("plugin", "version"),
);

CREATE TABLE "plugin_version_authors"(
    "plugin_id" INT NOT NULL,
    "plugin_version" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin", "plugin_version", "name"),
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("plugin", "version"),
);

CREATE TABLE "plugin_version_permissions"(
    "plugin_id" INT NOT NULL,
    "plugin_version" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR NOT NULL,
    PRIMARY KEY ("plugin", "plugin_version"),VARCHAR
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("plugin", "version"),
);

CREATE TABLE "plugin_version_commands"(
    "plugin_id" INT NOT NULL,
    "plugin_version" VARCHAR NOT NULL,
    "syntax" VARCHAR NOT NULL,
    "description" VARCHAR NOT NULL,
    "permission" VARCHAR NULL,
    PRIMARY KEY ("plugin", "plugin_version"),
    FOREIGN KEY ("plugin", "plugin_version") REFERENCES "plugin_versions"("plugin", "version"),
    FOREIGN KEY ("plugin", "plugin_version", "permission") REFERENCES "plugin_version_permissions"("plugin", "plugin_version", "name"),
);

CREATE TABLE "plugin_owners"(
    "plugin" INT NOT NULL,
    "user" INT NOT NULL,
    PRIMARY KEY ("plugin", "user"),
    FOREIGN KEY ("plugin") REFERENCES "plugins"("id"),
    FOREIGN KEY ("user") REFERENCES "users"("id"),
);

CREATE TABLE "users"(
    "id" SERIAL NOT NULL,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY ("id"),
)
```

```sql
SELECT SUM("downloads") FROM "plugin_versions" WHERE "plugin_version"."plugin" = $plugin_id;
SELECT SUM("stars") FROM "plugin_versions" WHERE "plugin_version"."plugin" = $plugin_id;

SELECT DISTINCT "plugins"."name", "plugin_versions"."version", (SELECT SUM("downloads") FROM "plugin_versions" WHERE "plugin_version"."plugin" = "plugin"."id") FROM "plugin", "plugin_versions" WHERE "plugin_version"."plugin" = "plugin"."id";

```