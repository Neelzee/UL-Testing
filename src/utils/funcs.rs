pub fn get_url(page: u32) -> String {
    let _ = "https://www.vinmonopolet.no/vmpws/v2/vmp/search?fields=FULL&pageSize=24&searchType=product&currentPage=0&q=:relevance:mainCategory:brennevin:mainSubCategory:brennevin_whisky";
    let s = "https://www.vinmonopolet.no/vmpws/v2/vmp/search?fields=FULL&pageSize=24&searchType=product&currentPage=";
    let s_2 = "&q=:relevance:mainCategory:brennevin:mainSubCategory:brennevin_whisky";
    let p = format!("{}{}{}", s, page, s_2);

    return p;
}

pub fn get_data_url(whisky: u32) -> String {
    format!(
        "https://www.vinmonopolet.no/vmpws/v3/vmp/products/{}?fields=FULL",
        whisky
    )
}
