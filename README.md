# Social Sports API

Simple social interactions API (posting, commenting, reacting) for sports. Defines clubs + teams, and their communities (fan clubs). Each community offers a feed of posts.

See [the protobuf schema](proto/api.proto) for an overview of available operations.

## Features
- Compatible with Serverless (aka lambda) execution
- Data stored in a Postgres-compatible database
- Event-driven
- Uses JWT Auth

## Design
### Architecture

Highlights:
- DDD with hexagon architecture
- Separated bounded contexts structure. This should allow for easy split into separate (micro-)services
- It utilizes the [Twelve-Factor](https://12factor.net/) best practices

### API
It's __gRPC__, because;
- API is only intended for internal use, for backend <> frontend communication
- protocol (contract) is explicitly expressed in IDLs (proto files)
- server + clients are automatically generated
- smaller payloads and faster than REST or GraphQL
- communication pattern (commands & queries) maps nicely onto RPC (as opposed to REST resources)
- straightforward versioning scheme

### Implementation language
It's __Rust__, because;
- able to express rich domain behavior and its constraints
- safe (no nulls, race conditions, corrupt memory...)
- fast
- small binaries (deployments)

### Implementation details
- Zoned DateTime in RFC3339, API as an epoch in milliseconds
- JWT tokens used in auth (e.g. determine current user) - validation is expected to be handled before

## FAQ
### Where are the validations?
The traditional validations are replaced by behavior-encoded types. For example, have a look at [club.rs](src/domain/club/aggregates/club_name.rs). This way behavior is always, and everywhere enforced.

Next, parsing is preferred above validation. See [this article](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) for more information.

Lastly, having separate types for IDs, names, etc. isn't just fanciness. It provides guarantees that supplied parameters/variables are correct, thus eliminating the need for many value checks (preconditions).

Many of the above design patterns are inspired or derived by ["making illegal state impossible"](https://fsharpforfunandprofit.com/posts/designing-with-types-making-illegal-states-unrepresentable/).

### What are these `probes`?
It's a way to separate business logic and observation code. Martin Fowler documented [it here](https://martinfowler.com/articles/domain-oriented-observability.html).

## Ideas, discussions, TODOs
- [ ] Allow upload images and generate image ids
- [ ] Publish events message bus
- [ ] Add referenced data in API queries
- [ ] `Team` and `Club` in one bounded context? (discussion)
- [ ] apply `policy` enforcement in use-cases (e.g. [community policies](src/domain/social/policies) )
- [ ] Event.kind should/could be derived
- [ ] Version aggregates
- [ ] Throttle api commands operations (idea)

