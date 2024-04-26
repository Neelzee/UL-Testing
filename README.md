# ua-testing

Adds test data to the different services

## How to use

```bash
cargo run -- <args>
```

## Args

- file
  Use `data.bin`, if it exsits, instead of making a new vinmonopol request

- db
  - Uploads the `Whiskies` to the database, will error if conflict on `ids`
    The database used is `host={DB_HOST} dbname=ulu_prod user=ulu_backend password={DB_PASSWORD}`
    where DB_HOST and DB_PASSWORD are environment variables.

- rec
  Serializes the `Vec<Whiskey>` into a json format, saving it as `whiskies.json`
