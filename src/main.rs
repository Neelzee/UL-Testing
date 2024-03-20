use postgres::{Client, NoTls};
use std::env;

use eyre::{Context, Result};
use request::get_all_whiskies;

mod request;
mod test;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let db_password =
        env::var("DB_PASSWORD").wrap_err("Failed finding enviroment variable `DB_PASSWORD`")?;
    let mut client = Client::connect(
        &format!("host=localhost:5432 database=ulu_prod user=ulu_backend password={db_password}"),
        NoTls,
    )?;

    for w in get_all_whiskies().await? {
        client
            .execute(
                "INSERT INTO whisky (img, title, price, summary, volume, percentage) VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &w.img,
                    &w.title,
                    &w.price,
                    &w.summary,
                    &w.volume,
                    &(w.percentage as i64),
                ],
            )
            .wrap_err("Failed writing to db")?;
    }

    Ok(())
}
