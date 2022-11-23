# Vertical Slice Architecture with Rust

This is an example on how to implement the Vertical Slice Architecture,
plus a basic CQRS, using Rust language. It's a simplistic way to do it, only
to provide a little directions on how you can do it and use it as a template to start a web api project.

## Tests

You can add an `RUST_LOG=debug` before the command to see more informations about the tests

Running integration tests:

```bash
cargo test --package backend-api --test tests --all-features
```

Running unit tests:

```bash
cargo test --package backend-api --lib --all-features
```

## Useful commands

### Running the API with debug logging:

```bash
RUST_LOG=debug cargo run
```

### Create new migrations

```bash
sea-orm-cli migrate generate name_of_migration
```

...and then move the migration to a specific DB type folder

### Updating database

The migrations are automatically runned by the application at startup but you can use:

```bash
DATABASE_URL={connection_string} sea-orm-cli migrate up
```

### Revert a migration

```bash
DATABASE_URL={connection_string} sea-orm-cli migrate down
```

### Generate a new entity

```bash
sea-orm-cli generate entity -s public -u {connection_string} -o ./entity/src -v
```

## TO-DO API

- [x] server side securities
  - [x] removing legacy headers
  - [x] adding security headers
  - [x] JSON parser
- [x] architecture
  - [x] CQRS
  - [x] ORM
  - [x] RESTful
  - [x] Logging -> Events, warnings, errors
- [x] authentication
  - [x] login
  - [x] validate jwt
  - [x] refresh token
- [ ] users
  - [x] Create
  - [x] Request
  - [ ] Update
  - [ ] Delete
- [ ] gallery
  - [ ] Create
  - [ ] Request
  - [ ] Update
  - [ ] Delete
  - [ ] Ordering
- [ ] photos
  - [ ] Create
  - [ ] Request
  - [ ] Update
  - [ ] Delete
  - [ ] Ordering

## TO-DO Features

- [x] Docker
- [x] Better custom errors
- [x] Better error handling
- [x] Middlewares (Error handling, Auth, Validations...)
- [x] Separate C/Q validations from C/Q Handlers
- [ ] Documentation

## Give it a try (improvements)

- [ ] Try GraphQL
- [ ] Try some events
- [ ] Try some crosscutting (MQ, Storage, Functions...)
- [ ] Try scheduled tasks and recurrent jobs
- [ ] Try some cache (noSQL, Redis or something)
