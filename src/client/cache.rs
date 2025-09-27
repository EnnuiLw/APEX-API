use bytes::Bytes;
use chrono::{Duration, prelude::*};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::Mutex;

const MAX_CAPACITY: usize = 50;
pub static MODEL_HASH: Lazy<Mutex<HashMap<String, Holder<Bytes>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct Holder<T> {
    target: T,
    expire_at: DateTime<Utc>,
}

impl<T> Holder<T>
where
    T: Clone,
{
    pub fn new(target: T, expire_at: DateTime<Utc>) -> Self {
        Self { target, expire_at }
    }

    fn get_target(&self, now: &DateTime<Utc>) -> Option<T> {
        if self.expire_at > *now {
            return Some(self.target.clone());
        }

        None
    }

    async fn extract_expire_data(target: &Mutex<HashMap<String, Holder<T>>>) -> Option<String> {
        let hash = target.lock().await;
        let now = Utc::now();

        for (key, data) in hash.keys().zip(hash.values()) {
            if data.expire_at <= now {
                return Some(key.to_string());
            }
        }

        None
    }

    async fn inner_get(
        key: &str,
        now: &DateTime<Utc>,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
    ) -> Option<T> {
        let hash = mutex.lock().await;
        if let Some(holder) = hash.get(key) {
            return holder.get_target(now);
        }

        None
    }

    async fn inner_set(
        key: &str,
        target: T,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
        expire_at: DateTime<Utc>,
    ) {
        let mut hash = mutex.lock().await;
        if hash.capacity() > MAX_CAPACITY {
            Self::extract_expire_data(mutex)
                .await
                .map(|k| hash.remove(&k));
        }

        hash.insert(key.to_string(), Holder::new(target, expire_at));
    }

    #[allow(unused)]
    pub async fn get(key: &str, mutex: &Mutex<HashMap<String, Holder<T>>>) -> Option<T> {
        let now = Utc::now();
        if let Some(target) = Self::inner_get(key, &now, &mutex).await {
            return Some(target);
        }

        None
    }

    #[allow(unused)]
    pub async fn set<Fut>(
        key: &str,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
        cache_expire_second_count: u64,
        f: impl FnOnce() -> Fut,
    ) where
        Fut: Future<Output = crate::ClientResult<T>>,
    {
        let now = Utc::now();
        let expire_at = now + Duration::seconds(cache_expire_second_count as i64);
        if let Ok(target) = f().await {
            Self::inner_set(key, target, mutex, expire_at).await;
        }
    }

    pub async fn get_and_set<Fut>(
        key: &str,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
        cache_expire_second_count: u64,
        f: impl FnOnce() -> Fut,
    ) -> crate::ClientResult<T>
    where
        Fut: Future<Output = crate::ClientResult<T>>,
    {
        let now = Utc::now();
        {
            if let Some(target) = Self::inner_get(key, &now, mutex).await {
                return Ok(target);
            }
        }

        let target = f().await?;
        Self::inner_set(
            key,
            target.clone(),
            mutex,
            now + Duration::seconds(cache_expire_second_count as i64),
        )
        .await;
        Ok(target)
    }
}

pub fn gen_cache_key(url: &str) -> String {
    let mut vec: Vec<_> = url.split("/").collect();
    vec.reverse();
    vec.into_iter().take(2).collect::<Vec<_>>().join("=")
}
