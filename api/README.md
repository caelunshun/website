# Feather Website Api
This is the RestAPI that handles all the logic for https://feathermc.org, the api is located at https://api.feathermc.org.

## Getting started
First install the sqlx cli
### Install sqlx
`cargo install sqlx-cli`

### Clear the database and apply all migrations
`cargo sqlx database reset`

### Create new migration
`cargo sqlx migrate add`

### Aply new migrations
`cargo sqlx migrate run`

### Running the api
`cargo run`