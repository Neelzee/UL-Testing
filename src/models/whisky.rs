use serde::{Deserialize, Serialize};

use super::raw_whisky::RawWhisky;

#[derive(Debug, Deserialize, Serialize)]
pub struct Whiskey {
    pub id: u32,
    pub img: String,
    pub percentage: u32,
    pub price: f32,
    pub rating: f32,
    pub summary: String,
    pub title: String,
    pub volume: f32,
}

impl Whiskey {
    pub fn new(
        id: u32,
        img: String,
        percentage: u32,
        price: f32,
        rating: f32,
        summary: String,
        title: String,
        volume: f32,
    ) -> Self {
        Whiskey {
            id,
            img,
            percentage,
            price,
            rating,
            summary,
            title,
            volume,
        }
    }

    pub fn from_raw(mut raw: RawWhisky) -> Self {
        Whiskey {
            id: 0,
            img: raw.images.pop().expect("Expected an image").url,
            percentage: raw.alcohol.value as u32,
            price: raw.price.value,
            rating: 0f32,
            summary: raw.summary,
            title: raw.name,
            volume: raw.volume.value,
        }
    }
}
