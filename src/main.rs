use tokio_postgres::{Client, NoTls};
use std::env;
use ua_rlib::models::whisky::Whiskey;

use eyre::{Context, Result};
use request::get_all_whiskies;

mod request;
mod test;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
 let db_password =
        env::var("DB_PASSWORD").wrap_err("Failed finding enviroment variable `DB_PASSWORD`")?;
let (client, connection) =
        tokio_postgres::connect(&format!("host=localhost dbname=ulu_prod user=ulu_backend password={db_password}"), NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    let mut i: i64 = 100;
    for w in get_all_whiskies().await? {
   

client
            .execute(
                "INSERT INTO whiskey (id, img, title, price, summary, volume, percentage, avg_score) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[
                    &i,
                    &w.img,
                    &w.title,
                    &(w.price as f64),
                    &w.summary,
                    &(w.volume as f64),
                    &(w.percentage as f64),
                    &(0f64)
                ],
            )
            .await
            .wrap_err("Failed writing to db")?;
        i += 1;
    }
    Ok(())
}

