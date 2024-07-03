use candid::CandidType;
use serde::Deserialize;

const DEFAULT_PAGE_SIZE: u32 = 5;
const MAX_PAGE_SIZE: u32 = 50;

#[derive(CandidType, Deserialize)]
pub struct PageInfo {
    page: u32,
    pub(crate) page_size: usize,
    pub(crate) offset: usize,
}

impl PageInfo {
    pub fn new(page: Option<u32>, page_size: Option<u32>) -> Self {
        let page = page.unwrap_or(1u32);
        let page_size = u32::min(page_size.unwrap_or(DEFAULT_PAGE_SIZE), MAX_PAGE_SIZE);
        let offset = ((page - 1) * page_size) as usize;

        Self {
            page,
            page_size: page_size as usize,
            offset,
        }
    }
}
