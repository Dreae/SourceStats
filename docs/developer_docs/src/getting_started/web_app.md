# The Web App
Next up you'll need to set up your environment for running the SourceStats web
application.

The web application is what handles the processing and storing of all in-game
events from the game servers, as well as rendering the web interface for players
to browse their game performance.

### PostgreSQL
SourceStats requires PostgreSQL version 10. Visit the [PostgreSQL website](https://www.postgresql.org/download/)
and install the version appropriate for your operating system. Make sure you install version 10.
#### Tip: keep track of the root password for your Postgres installation, you'll need it later

### TimeScaleDB
SourceStats uses __TimeScaleDB__ on top of Postgres for storing time-series data
such as gameplay events. Head over tothe [TimeScaleDB website](https://www.timescale.com/)
and follow the instructions for installing for your platform. Make sure you follow
all steps for installing TimeScaleDB.

### Installing Diesel
SourceStats uses [Diesel](https://diesel.rs/) for querying the database and running migrations.
You will need the Diesel CLI to run database migrations for SourceStats
```bash
cargo install diesel_cli --no-default-features --features postgres
```
Next you need to tell the Diesel CLI how to connect to your database. Copy `.env.default` to
`.env` and replace `psql_username` and `psql_password` with the username and password for your
Postgres installation.

### Creating the SourceStats Database
Once you have Postgres and TimeScaleDB installed, you will need to create the
database for SourceStats.

First, connect to your new Postgres instance:
```bash
psql -U postgres
```

Then create a new database called `sourcestats` and switch to it
```
postgres=# create database sourcestats;
CREATE DATABASE
postgres=# \c sourcestats
```

Now enable the TimeScaleDB extension for your new database:
```
sourcestats=# CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
```
You are now ready to run the migrations to set up your SourceStats database
```
diesel migration run
```