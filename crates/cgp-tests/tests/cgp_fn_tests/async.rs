use cgp::prelude::*;

#[cgp_fn]
#[async_trait]
pub async fn greet(&self, #[implicit] name: &str) -> String {
    format!("Hello, {}!", name)
}
