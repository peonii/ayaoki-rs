use anyhow::Result;
use std::{env, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")?;

    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT;

    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

    let http = Arc::new(HttpClient::new(token));

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                println!("Error receiving event {:?}", source);

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        cache.update(&event);

        tokio::spawn(twl_ayaoki::events::handler::handle_event(
            event,
            Arc::clone(&http),
        ));
    }

    Ok(())
}
