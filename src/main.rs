use std::{
    fs::File,
    io::{BufWriter, Write},
};

use eyre::Result;
use request::get_all_whiskies;

mod request;
mod test;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let file = File::create("test.csv")?;

    let mut writer = BufWriter::new(file);

    for w in get_all_whiskies().await? {
        writer.write_all(
            format!(
                "{};{};{};{};{};{};{};{};{}",
                w.id,
                w.img,
                w.percentage,
                w.price,
                w.rating,
                w.rating,
                w.summary,
                w.title,
                w.volume
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}
