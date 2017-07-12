# wiko
A wiki for documenting code bases and sharing useful snippets with coworkers.

## Setup
Database was set up following the [Diesel getting started guide](http://diesel.rs/guides/getting-started/).  First, install `diesel_cli` to perform database migrations:

    cargo install diesel_cli

Then execute the actual database migration:

    diesel migration run


