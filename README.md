## Zagreus - The identity provider service that wants to break free

### Prerequisites

1. You'll need Docker, Node, and Rust installed

2. Install [SQLx](https://github.com/launchbadge/sqlx):

```
cargo install sqlx-cli
```

### Installation

1. Run the [Hydra](https://www.ory.sh/hydra/docs/) instance (necessary each time the docker containers are shut down):

```
docker-compose up hydra
```

2. You'll need to setup your database. Zagreus uses [SQLx](https://github.com/launchbadge/sqlx) to handle database management, migrations, and queries.

```
sqlx database create
sqlx migrate run
```

3. Build the app using Cargo (should be installed with Rust):

```
cargo build --release
```

_Building in release mode might take some time, relax and grab some coffee :grin: Dev mode with `cargo check` is much, much faster._

4. Follow the client's instructions or create your own client (see [here](./docs/create-client.md) for more)

_From that step it's up to the client to elaborate its own building process, but here are some things to know_.

- The `zagreus` command is meant to be used as a standalone binary installed on a system or via Docker. It relies on environment variables that have to be present in a `.env` file at the root of the _client_ project. This way the `zagreus` binary can load the required configuration and run properly.
- In order to initialize your client you can run the following command (needed once):

```
zagreus init --client-name [my-client-name]
```

- Zagreus can be started using this command:

```
zagreus run
```

Or by prepending `RUST_LOG=info` to the command above to display the logs in the terminal.

### Documentation

Documentation can be found [here](./docs).
