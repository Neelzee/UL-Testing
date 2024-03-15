use std::future::IntoFuture;

use eyre::{Context, ContextCompat, Result};
use reqwest::{Client, StatusCode};
use serde_json::Value;

use crate::{
    models::{raw_whisky::RawWhisky, whisky::Whiskey},
    utils::{
        consts::{get_url, WHISKY_PAGE_COUNT},
        funcs::get_data_url,
    },
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
pub async fn get_ids(client: &Client, page: u32) -> Result<Vec<String>> {
    let request = client.get(get_url(page)).build()?;

    serde_json::from_str::<Value>(&client.execute(request).await?.text().await?)
        .wrap_err("Failed parsing")
        .and_then(|val| {
            val["productSearchResult"]["products"]
                .as_array()
                .wrap_err("Missing field on json")
                .and_then(|vec| {
                    Ok(vec
                        .into_iter()
                        .map(|id| id["code"].to_string().replace("\\", "").replace("\"", ""))
                        .collect::<Vec<String>>())
                })
        })
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

pub async fn get_all_whiskies() -> Result<Vec<Whiskey>> {
    let client = Client::default();

    let mut res = Vec::new();

    for i in 0..WHISKY_PAGE_COUNT {
        let ids = get_ids(&client, i)
            .await
            .wrap_err("Failed getting IDs")
            .and_then(|raw_ids| {
                Ok(raw_ids
                    .into_iter()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>())
            })?;

        for id in ids {
            res.push(get_whisky_data(&client, id).await?);
        }
    }

    Ok(res)
}
