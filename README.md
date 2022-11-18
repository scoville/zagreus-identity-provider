## Zagreus - The identity provider service that wants to break free

### Prerequisites

1. You'll need Docker, Node, and Rust installed

2. Install [SQLx](https://github.com/launchbadge/sqlx):

```
cargo install sqlx-cli
```

### Installation

1. Set up a local postgres database for the idp service and run the [Hydra](https://www.ory.sh/hydra/docs/) instance (necessary each time the docker containers are shut down):

```
docker-compose up zagreus-postgres hydra
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

4. If needed, add the built executable to your system's $PATH. Easiest way would be to [symlink the built binary](https://apple.stackexchange.com/a/41586) to one folder that is already in your path, e.g. /usr/local/bin/

```bash
echo $PATH

sudo ln -s /[full-path-to-the-project-folder]/zagreus-identity-provider/target/debug/zagreus /usr/local/bin/
```

5. Go to [the client project](https://github.com/scoville/zagreus-identity-provider-ats) and run the zagreus commands to initiate the idp database with a client and run the server.

6. Follow the client's instructions or create your own client (see [here](./docs/create-client.md) for more)

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
