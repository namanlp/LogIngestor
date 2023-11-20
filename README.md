# Log Ingestor and Query Interface:

Log Ingestor and Query Interface for easily monitoring your logs in Rust Language.

## Built With

- ![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
- ![](https://img.shields.io/badge/MariaDB-003545?style=for-the-badge&logo=mariadb&logoColor=white)
- ![](https://img.shields.io/badge/GIT-E44C30?style=for-the-badge&logo=git&logoColor=white)

## Features

Here I have listed some features for the Log Ingestor

### Log Ingestor

- [x] Ingest Logs on port `3000` efficiently and store them in a database.
- [x] Ensures scalability to handle high volumes of logs efficiently.
- [x] Mitigate potential bottlenecks by asynchronous execution.

### Query Interface:

- [x] Include Various filters for filtering logs.
- [x] Search within specific time stamps.
- [x] Efficient and quick search results.
- [x] Intuitive CLI based User Interface.
- [x] Real-time log ingestion and searching capabilities.

### Dependencies

1. **Rust and Cargo :** Please Ensure that Rust and Cargo are installed and running.
2. **MySql or Mariadb :** Install MariaDB or MySQL and ensure that it is running properly.


### Getting Started

Here is guide to install and start using this Log Ingestor and Query Interface. 

1. **Clone Repository :** First of all, you have to clone this Repository. It is easiest to clone it using git
```shell
git clone https://github.com/namanlp/LogIngestor.git
```
and then `cd` into the folder just created.

```shell
cd LogIngestor
```

2. **Add .env :** You must add a database URL to `.env` that you may be using for storing the logs. URL must be in below format for both MariaDB and MySQL. 

`mysql://username:password@host:port/database_name`

For Example, URL for MariaDB/MySQL running on your PC may look like

`mysql://your_username:your_password@127.0.0.1/logs`

To create `.env` file, simply write

```shell
echo DATABASE_URL=mysql://your_username:your_password@127.0.0.1/logs > .env
```

3. **Run :** Finally, just run the program using
```shell
cargo run
```

and boom! The Cargo will automatically install necessary crates and run the Log Ingestor in background.

4. You can now see Query Interface, you can ask for Queries in this, which we will see further.

