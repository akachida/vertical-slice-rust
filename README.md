# Vertical Slice Architecture with Rust


This is a simple example on how to implement the Vertical Slice Architecture,
plus a basic CQRS, using Rust language. It's a simplistic way to do it, only
to provide a little directions on how you can do it.

Running tests:
```
cargo test --package vsa-rust --test tests --all-features
```

## TO-DO

- [ ] Docker
- [ ] Better custom errors
- [ ] Better error handling
- [ ] Middlewares (Error handling, Auth, Validations...)
- [ ] Separate C/Q validations from C/Q Handlers


## Give it a try (improvements)

- [ ] Try GraphQL
- [ ] Try some events
- [ ] Try some crosscutting (MQ, Storage, Functions...)
- [ ] Try scheduled tasks and recurrent jobs
- [ ] Try some cache (noSQL or something)
