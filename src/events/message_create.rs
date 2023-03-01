use anyhow::Result;
use std::sync::Arc;
use twilight_http::Client as HttpClient;

type Message = std::boxed::Box<twilight_model::gateway::payload::incoming::MessageCreate>;

pub async fn handle(http: Arc<HttpClient>, msg: Message) -> Result<()> {
    if msg.content == "ayaoki" {
        http.create_message(msg.channel_id)
            .content("that's me")?
            .await?;
    }

    Ok(())
}
