use crate::{MonoRepository, MonoStateCache, STREAM_NAME};
use anyhow::Result;
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::repository::Repository;
use kurrentdb::Client;
use redis::Client as Redis;

const GROUP_NAME: &str = "state";

pub async fn cache_state(redis_client: Redis, event_store_db: Client) -> Result<()> {
    let dto_redis = MonoStateCache::new(redis_client.clone());

    let repo_state = MonoRepository::new(event_store_db, dto_redis.clone());

    let stream = Stream::Stream(STREAM_NAME);
    repo_state.cache_dto(&stream, GROUP_NAME).await?;

    Ok(())
}
