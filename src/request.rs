use std::future::IntoFuture;

use eyre::{Context, ContextCompat, OptionExt, Result};
use reqwest::{Client, StatusCode};
use serde_json::Value;

use crate::utils::{
    consts::WHISKY_PAGE_COUNT,
    funcs::{get_data_url, get_url},
};
use ua_rlib::models::{raw_whisky::RawWhisky, whisky::Whiskey};

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
    let request = client
        .get(get_url(page))
        .build()
        .wrap_err("Failed building request")?;
    let response = client
        .execute(request)
        .await
        .wrap_err("Failed executing request")?;
    let body = response
        .text()
        .await
        .wrap_err("Failed getting response-body")?;
    let json = serde_json::from_str::<Value>(&body)
        .wrap_err(format!("Failed getting json from `{body:?}`"))?;
    let raw_ids = json["productSearchResult"]["products"]
        .as_array()
        .ok_or_eyre("Missing field on json")?;
    let parsed_ids = raw_ids
        .into_iter()
        .map(|id| id["code"].to_string().replace("\\", "").replace("\"", ""))
        .collect::<Vec<String>>();

    Ok(parsed_ids)
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
                .and_then(|e| {
                    serde_json::from_str::<Value>(&e)
                        .wrap_err(format!("Failed parsing json, `{e:?}`"))
                })
                .and_then(|rw| Whiskey::from_value(rw))
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
