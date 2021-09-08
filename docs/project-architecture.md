## Project Architecture

### Introduction

Zagreus is divided into several folders that are mostly Rust's [_crates_](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html). Each folder is in charge of a layer of the application and you can think of them as the different layers in the [Clean architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html). We'll see more into details what the crates in the "Crates" section below. The main advantage of crates over simple folders is the great flexibily and reusability they offer.

### Stack

- [Rust](https://www.rust-lang.org) on the backend (API and webapp)
- [Actix](https://actix.rs/) for concurrenty and routing
- [SQLx](https://github.com/launchbadge/sqlx) to interact with the database
- [Tera](https://tera.netlify.app/) as the template engine ([Askama](https://djc.github.io/askama/) is a possible alternative if it offers all the flexibility we need to make Zagreus usable by any other applications)
- [AlpineJS](https://alpinejs.dev/) for the frontend as everything is server side rendered in Rust we only need some logic on top to handle form validations and basic interaction. The other benefit is that it's easy for other applications to use it + html as it's not as opinionated as React or Vue for instance

### Crates

#### `zagreus-domain`

A library that handles all the interaction with the database. It also contains all the models.

#### `zagreus-config`

A very small and simple library that loads a `.env` file and exposes it as global variables. It must be initialize using the `read` function.

#### `zagreus`

The main API binary.

### Future

Update docker related files.
