# Endpoints
The base url for all endpoints are
`https://api.feathermc.org/v1`

# `/me`
## `get`
Returns user data regarding the logged in user
### `headers`
- `Authorization`

```json
{
    "id": 69,
    "name": "Jacob Emil Ulvedal Rosborg",
    "github_id": 1496019,
    "github_account": "Defman",
    "email": { 
        "address": "jacob@rosborg.dk", 
        "verified": true
    },
    "tokens": [
        {
            "id": 0,
            "name": "windows",
            "created": 0,
            "used_total": 42
        }
    ]
}
```

## `post`
Login the new user, if not exists creates a new user. The `access_token` is from GitHub OAuth redirect.
```json
{
    "access_token": "<token value>"
}
```
returns the same as `get` on `/me`

## `delete`
Deletes the user, and all self owned plugins
### `headers`
- `Authorization`

# `/me/tokens/:token_name`

## `get`
Creates and returns a new token
### `headers`
- `Authorization`

returns
```json
{
    "id": 1,
    "name": "macbook",
    "created": 0,
    "token": "<token value>",
    "used_total": 0
}
```

## `delete`
Deletes the token
### `headers`
- `Authorization`

# `/me/email`
We should support chaning email, requesting verification code, and submitting verification code.
## `put`
Change email


# `/me/email/verify`
## `get`
Send verification code
### `headers`
- `Authorization`

# `/me/email/verify/:code`
## `get`
Verify email by code


# `/users`
## `get`

### `query parameters`
- `search`
- `page`

returns
```json
[
    {
        "id": 69,
        "name": "Jacob Emil Ulvedal Rosborg",
        "github_id": 42,
        "github_account": "Defman"
    }
]
```

# `/users/:id`
## `get`

### `query parameters`
- `page`
```json
{
    "id": 69,
    "name": "Jacob Emil Ulvedal Rosborg",
    "github_id": 42,
    "github_account": "Defman",
    "plugins_page": 0,
    "plugins_pages": 10,
    "plugins": [
        {
            "id": "world-edit",
            "name": "WorldEdit",
            "description": "WorldEdit is an in-game map editor for both creative and survival",
            "downloads": 420000,
            "downloads_recent": 690,
            "last_updated": 20,
            "links": {
                "homepage": "https://google.com",
                "repository": "https://github.com"
            },
            "versions": ["1.0.4", "1.13"]
        }
    ]
}
```

# `/plugins`

## `get`
List all plugins, can be modified with query paremeters
### `query parameters`
- `search`
- `categories`
- `page`
- `owners`

```json
[
        {
            "id": "world-edit",
            "name": "WorldEdit",
            "description": "WorldEdit is an in-game map editor for both creative and survival",
            "downloads_total": 420000,
            "downloads_total_recent": 690,
            "last_updated": 20,
            "links": {
                "homepage": "https://google.com",
                "repository": "https://github.com"
            },
            "versions": ["1.0.4", "1.13"]
        },
        {
            "id": "world-guard",
            "name": "WorldGuard",
            "description": "WorldGuard is an in-game map editor for both creative and survival",
            "downloads": 420000,
            "downloads_recent": 690,
            "last_updated": 20,
            "versions": ["1.16"]
        },
        {
            "id": "essentials",
            "name": "Essentials",
            "description": "WorldEdit is an in-game map editor for both creative and survival",
            "downloads": 420000,
            "downloads_recent": 690,
            "last_updated": 20,
            "versions": ["1.0.0"]
        },
    ]
```

## `post`
Published a new plugin version
### `headers`
- `Authorization`

### `body`
- `len(metadata) as u32`
- `metadata`
- `len(avatar) as u32`
- `avatar`
- `len(plugin_tars)`
    - `len(plugin_tar_metadata)`
    - `plugin_tar_metadata`
    - `len(plugin_tar)`
    - `plugin tar`

# `/plugins/:plugin`
## `get`
```json
{
    "name": "world-edit",
    "owners": [
        {
            "id": 42,
            "name": "Jacob Emil Ulvedal Rosborg",
            "github_id": "1496019",
            "github_name": "Defman"
        }
    ],
    "versions": ["1.0.0", "0.5", "0.1"],
    "downloads": 690,
    "downloads_recent": 420,
    "links": {
        "homepage": "https://google.com",
        "repository": "https://github.com"
    }
}
```

# `/plugins/:plugin/invite/:user_id`
## `get`
Invites a user to be an owner of this plugin
### `headers`
- `Authorization`


# `/plugin/:plugin/:version`
## `get`
```json
{
    "description": "WorldEdit is an in-game map editor for both creative and survival",
    "categories": [],
    "dl": []
}
```

# Experimental
# `/plugin/:plugin/:version/dependencies`

## `get`
```json
{
    "components": [
        {
            "name": "Health",
            "version": 0,
            "fields": ["f32"]
        },
        {
            "name": "Position",
            "version": 1,
            "fields": [
                {
                    "name": "x",
                    "type": "f32",
                },
                {
                    "name": "y",
                    "type": "f32"
                },
                {
                    "name": "z",
                    "type": "f32"
                }
            ]
        },
        {
            "name": "EntityType",
            "variants": [
                "Zombie",
                "Skeleton",
            ]
        }
    ],
    "functions": [
        {
            "name": "get_heatlh",
            "version": 2,
            "parameters": ["u32"],
            "return": {
                "name": "Health",
                "fields": ["f32"]
            }
        }
    ]
}
```

# `/plugin/:plugin/:version/exposes`
```json
{
    "components": [
        {
            "name": "Health",
            "version": 0,
            "fields": [ {
                "type": "f32",
                "visability": "private",
            }]
        },
        {
            "name": "Position",
            "version": 1,
            "fields": [
                {
                    "name": "x",
                    "type": "f32",
                },
                {
                    "name": "y",
                    "type": "f32"
                },
                {
                    "name": "z",
                    "type": "f32"
                }
            ]
        },
        {
            "name": "EntityType",
            "variants": [
                "Zombie",
                "Skeleton",
            ]
        }
    ],
    "functions": [
        {
            "name": "heatlh_get",
            "version": 2,
            "parameters": ["u32"],
            "return": {
                "name": "Health",
                "fields": ["f32"]
            }
        },
        {
            "name": "heatlh_new",
            "version": 2,
            "parameters": ["f32"],
            "return": {
                "name": "Result",
                "variants": [
                    {
                        "name": "Err",
                        "fields": ["String"],
                    },
                    {
                        "name": "Ok",
                        "fields": [
                            {
                                "name": "Health",
                                "fields": ["f32"]
                            }
                        ]
                    }
                ]
            }
        },
    ]
}
```