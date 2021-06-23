use profanity_filter::PROFANITY_FILTER;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check(s: &str) -> bool {
    PROFANITY_FILTER.check(s)
}

#[wasm_bindgen]
pub fn censor(s: &str) -> String {
    PROFANITY_FILTER.censor(s)
}

#[wasm_bindgen]
pub fn find(s: &str) -> String {
    PROFANITY_FILTER.find(s).collect::<Vec<_>>().join(", ")
}
