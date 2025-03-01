use baryonyx::*;

#[tokio::main]
async fn main() -> Result<(), Err> {
    ServerBuilder::new()
        .with_port(3000)?
        .with_content_limit(1024 * 1024 * 1)?
        .with_page("/", Page::new().with_content("Hello World".into()))?
        .with_page("/about", Page::new().with_content("About".into()))?
        .start()
        .await?;

    Ok(())
}
