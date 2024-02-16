pub const URL: &str = "https://www.vinmonopolet.no/search?searchType=product&q=%3Arelevance%3AmainCategory%3Abrennevin%3AmainSubCategory%3Abrennevin_whisky";

pub fn page_n(p: u32) -> String {
    format!("https://www.vinmonopolet.no/search?searchType=product&currentPage={}&q=%3Arelevance%3AmainCategory%3Abrennevin%3AmainSubCategory%3Abrennevin_whisky", p)
}

pub const _URL: &str = "https://www.vinmonopolet.no/vmpws/v2/vmp/products/";
