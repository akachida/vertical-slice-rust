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

## TO-DO

- [ ] Docker
- [x] Better custom errors
- [x] Better error handling
- [x] Middlewares (Error handling, Auth, Validations...)
- [x] Separate C/Q validations from C/Q Handlers

## Give it a try (improvements)

- [ ] Try GraphQL
- [ ] Try some events
- [ ] Try some crosscutting (MQ, Storage, Functions...)
- [ ] Try scheduled tasks and recurrent jobs
- [ ] Try some cache (noSQL, Redis or something)
