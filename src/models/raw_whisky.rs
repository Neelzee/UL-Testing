use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RawWhisky {
    #[serde(rename = "ageLimit")]
    pub age_limit: u32,
    pub alcohol: Alcohol,
    #[serde(rename = "bioDynamic")]
    pub bio_dynamic: bool,
    pub buyable: bool,
    pub code: String,
    pub color: String,
    pub content: Content,
    pub description: String,
    pub distributor: String,
    #[serde(rename = "distributorId")]
    pub distributor_id: usize,
    pub district: District,
    pub eco: bool,
    #[serde(rename = "environmentalPackaging")]
    pub environmental_packaging: bool,
    pub expired: bool,
    #[serde(rename = "fairTrade")]
    pub fair_trade: bool,
    pub gluten: bool,
    pub images: Vec<Image>,
    #[serde(rename = "isGoodFor")]
    pub is_good_for: Vec<String>,
    pub kosher: bool,
    #[serde(rename = "litrePrice")]
    pub litre_price: LitrePrice,
    pub main_category: MainCategory,
    pub main_country: MainCountry,
    pub main_producer: MainProducer,
    pub main_sub_category: MainSubCategory,
    pub main_sub_sub_category: MainSubSubCategory,
    pub method: String,
    pub name: String,
    #[serde(rename = "packageType")]
    pub package_type: String,
    pub price: Price,
    pub product_selection: String,
    pub raastoff: Vec<Raastoff>,
    #[serde(rename = "releaseMode")]
    pub release_mode: bool,
    #[serde(rename = "similarProducts")]
    pub similar_products: bool,
    pub smell: String,
    pub status: String,
    #[serde(rename = "statusNotification")]
    pub status_notification: bool,
    pub summary: String,
    pub sustainable: bool,
    pub taste: String,
    pub url: String,
    pub volume: Volume,
    #[serde(rename = "wholeSaler")]
    pub whole_saler: String,
    pub year: String,
}

#[serde(rename = "alcohol")]
#[derive(Deserialize, Serialize)]
pub struct Alcohol {
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "readableValue")]
    pub readable_value: String,
    pub value: f32,
}

#[serde(rename = "content")]
#[derive(Deserialize, Serialize)]
pub struct Content {
    pub ingredients: Vec<Ingredients>,
    pub traits: Vec<Traits>,
}

#[serde(rename = "content")]
#[derive(Deserialize, Serialize)]
pub struct Ingredients {
    pub code: String,
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "readableValue")]
    pub readable_value: String,
}

#[serde(rename = "traits")]
#[derive(Deserialize, Serialize)]
pub struct Traits {
    pub name: String,
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "readableValue")]
    pub readable_value: String,
}

#[serde(rename = "district")]
#[derive(Deserialize, Serialize)]
pub struct District {
    pub code: String,
    pub name: String,
    #[serde(rename = "searchQuery")]
    pub search_query: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "altText")]
    pub alt_text: String,
    pub format: String,
    #[serde(rename = "imageType")]
    pub image_type: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct LitrePrice {
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "readableValue")]
    pub readable_value: String,
    pub value: f32,
}

#[derive(Deserialize, Serialize)]
pub struct MainCategory {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct MainCountry {
    pub code: String,
    pub name: String,
    #[serde(rename = "searchQuery")]
    pub search_query: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct MainProducer {
    pub code: String,
    pub name: String,
    #[serde(rename = "searchQuery")]
    pub search_query: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct MainSubCategory {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct MainSubSubCategory {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Price {
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "readableValue")]
    pub readable_value: String,
    pub value: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Raastoff {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Volume {
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "readableValue")]
    pub readable_value: String,
    pub value: f32,
}
