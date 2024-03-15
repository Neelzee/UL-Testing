pub const VM_URL: &str = "https://www.vinmonopolet.no/search?searchType=product&currentPage=1&q=%3Arelevance%3AmainCategory%3Abrennevin%3AmainSubCategory%3Abrennevin_whisky";

pub const VM_URL_FH: &str = "https://www.vinmonopolet.no/search?searchType=product&currentPage=";
pub const VM_URL_SH: &str =
    "&q=%3Arelevance%3AmainCategory%3Abrennevin%3AmainSubCategory%3Abrennevin_whisky";

pub fn get_url(page_nr: u32) -> String {
    format!("{}{}{}", VM_URL_FH, page_nr, VM_URL_SH)
}

pub const TEST_REQUEST_RATE_LIMIT: f32 = 0.5f32;

pub const WHISKY_PAGE_COUNT: u32 = 52;
