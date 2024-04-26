use bincode::{deserialize_from, serialize};
use eyre::{Context, Result};
use request::get_all_whiskies;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use tokio_postgres::{Client, NoTls};
use ua_rlib::models::whisky::Whiskey;

mod request;
mod test;
mod utils;

async fn load_test_to_db(viksies: &Vec<Whiskey>) -> Result<()> {
    let db_password =
        env::var("DB_PASSWORD").wrap_err("Failed finding enviroment variable `DB_PASSWORD`")?;
    let (client, connection) = tokio_postgres::connect(
        &format!("host=localhost dbname=ulu_prod user=ulu_backend password={db_password}"),
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut i: i64 = 100;
    for w in viksies {
        client
            .execute(
                "INSERT INTO whiskey (id, img, title, price, summary, volume, percentage, avg_score) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[
                    &i,
                    &(w.img.replace("\"", "")),
                    &(w.title.replace("\"", "")),
                    &(w.price as f64),
                    &(w.summary.replace("\"", "")),
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

#[derive(Debug, Serialize, Deserialize)]
struct ImgPayload {
    id: usize,
    img: Vec<u8>,
}

async fn load_test_to_fiso(viskies: &Vec<Whiskey>) -> Result<()> {
    let client = reqwest::ClientBuilder::new().build()?;

    let mut i = 100;
    for w in viskies {
        let req = client
            .get(w.img.replace("\"", ""))
            .header("content_type", "image/jpeg")
            .build()?;
        let res = client.execute(req).await?.bytes().await?;

        let payload = ImgPayload {
            id: i,
            img: res.into(),
        };

        let req = client
            .post(format!("http://localhost:8001/api/img/{i}w"))
            .header("content_type", "image/jpeg")
            .build()?;
        let res = client.execute(req).await?;

        if res.status() != StatusCode::OK {
            println!("Got this status: {:?}", res.status());
        }

        i += 1;
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let viskies: Vec<Whiskey>;

    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.contains(&String::from("file")) {
        let file = File::open("data.bin")?;
        let reader = BufReader::new(file);
        viskies = deserialize_from(reader)?;
    } else {
        viskies = get_all_whiskies().await?;

        let mut file = File::create("data.bin")?;
        let buffer = serialize(&viskies)?;
        file.write_all(&buffer)?;
    }

    if args.contains(&String::from("rec")) {
        let file = File::create("whiskies.json")?;
        let mut writer = BufWriter::new(file);
        let buf = to_string_pretty(&viskies)?;
        writer.write_all(&mut buf.as_bytes())?;

        return Ok(());
    }

    if args.contains(&String::from("db")) {
        load_test_to_db(&viskies).await?;
    }

    if args.contains(&String::from("fiso")) {
        load_test_to_fiso(&viskies).await?;
    }

    Ok(())
}
