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


## Getting Started

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
5. **Stopping :** To stop the program, simply first enter `9` to stop Query Interface, and then <kbd>ctrl</kbd>+ <kbd>c</kbd> to stop Log Ingestor.

## Usage

### Log Ingestor

Now, you have to first send some logs to Log Ingestor to store it. You can do it by `curl` command or any other utility you like.

Example log

```json
{
	"level": "error",
	"message": "Failed to connect to DB",
    "resourceId": "server-1234",
	"timestamp": "2023-09-15T08:00:00Z",
	"traceId": "abc-xyz-123",
    "spanId": "span-456",
    "commit": "5e5342f",
    "metadata": {
        "parentResourceId": "server-0987"
    }
}
```

Example Command to send Logs is

```shell
curl -X POST -H "Content-Type: application/json" -d '{
       "level": "error",
       "message": "Failed to connect to DB",
       "resourceId": "server-1234",
       "timestamp": "2023-09-15T08:00:00Z",
       "traceId": "abc-xyz-123",
       "spanId": "span-456",
       "commit": "5e5342f",
       "metadata": {
         "parentResourceId": "server-0987"
       }
     }' http://127.0.0.1:3000/
```

And Voila! You just sent your first log to our Log Ingestor. You are now ready to use Query Interface.

**Note :** You have to send this from a separate terminal instance than Query Interface.

### Query Interface

After you have successfully stored the Log, you can successfully start using the query interface.

For this, you have to just select the desired filter and enter the filter text. 

For example, say you have to all the level `error` logs. Your interaction will look like

>1. Level
>2. Message
>3. ResourceId
>4. TimeStamp
>5. TraceId
>6. SpanId
>7. Commit
>8. Metadata.parentResourceId
>9. Exit <br />
>Choose the Option to Query Logs :

You should enter `1`. Then you would be asked for level

> Enter Level : 

As you want all Logs that are `error`, enter `error`.

You will see the logs with all details now.

> ("error", "Failed to connect to DB", "server-1234", "2023-09-15 08:00:00", "abc-xyz-123", "span-456", "5e5342f", "server-0987")

Similarly, if you want all the logs that were logged on or after 10 September 2023 and before 16 september, you can use 

`4`

`2023-09-10T00:00:00Z`

`2023-09-15T23:59:59Z`

> ("error", "Failed to connect to DB", "server-1234", "2023-09-15 08:00:00", "abc-xyz-123", "span-456", "5e5342f", "server-0987")


