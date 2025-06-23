use cgp::prelude::*;

#[cgp_handler]
async fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[cgp_handler]
async fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
    Ok(a + b)
}
