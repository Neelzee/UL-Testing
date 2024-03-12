use eyre::{Context, Result};
use reqwest::{Client, StatusCode};
use serde_json::Value;

use crate::{
    models::{raw_whisky::RawWhisky, whisky::Whiskey},
    utils::{consts::get_url, funcs::get_data_url},
};

pub async fn fetch_page(client: &Client, page_nr: u32) -> Result<String> {
    Ok(client.get(get_url(page_nr)).send().await?.text().await?)
}

/// Assumed structure of json value:
///
/// ```json
/// {
///     "productSearchResult": {
///         "products": [
///             {
///                 "code": "id"
///             },
///             //...
///         ]
///     }
/// }
/// ```
pub async fn get_ids(value: serde_json::Value) -> Result<Vec<String>> {
    let mut res = Vec::new();

    for el in value["productSearchResult"]["products"].as_array().ok_or(
        <serde_json::Error as serde::de::Error>::missing_field("field"),
    )? {
        res.push(el["code"].to_string().replace("\\", "").replace("\"", ""));
    }
    Ok(res)
}

pub async fn get_whisky_data(client: &Client, whisky: u32) -> Result<Whiskey> {
    let url = get_data_url(whisky);

    let request = client.get(url).build()?;
    match client.execute(request).await {
        Ok(response) if response.status() == StatusCode::OK => {
            return response
                .text()
                .await
                .wrap_err("Failed getting body")
                .and_then(|e| serde_json::from_str::<RawWhisky>(&e).wrap_err("Failed parsing json"))
                .and_then(|rw| Ok(Whiskey::from_raw(rw)))
        }
        Ok(response) => {
            eprintln!("{:?}", response);
            return Err(eyre::ErrReport::new(
                <serde_json::Error as serde::de::Error>::missing_field("Did not get 200 Response"),
            ));
        }
        Err(err) => {
            return Err(err.into());
        }
    }
}
