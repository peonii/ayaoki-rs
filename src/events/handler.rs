use anyhow::Result;
use std::sync::Arc;
use twilight_gateway::Event;
use twilight_http::Client as HttpClient;

pub async fn handle_event(event: Event, http: Arc<HttpClient>) -> Result<()> {
    match event {
        Event::MessageCreate(msg) => super::message_create::handle(http, msg).await,
        Event::Ready(_) => {
            println!("Shard is ready");
            Ok(())
        }
        _ => Ok(()),
    }?;

    Ok(())
}
