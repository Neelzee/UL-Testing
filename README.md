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
  Uploads the `Whiskies` to the database, will error if conflict on `ids`

- rec
  Serializes the `Vec<Whiskey>` into a json format, saving it as `whiskies.json`
