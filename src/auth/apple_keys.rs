use std::collections::HashMap;
use std::sync::Arc;

use reqwest;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
struct KeyResponse {
    keys: Vec<AppleKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleKey {
    pub kty: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

pub struct AppleKeyManager {
    keys: Arc<RwLock<HashMap<String, AppleKey>>>,
}

impl AppleKeyManager {
    pub fn new() -> Self {
        AppleKeyManager {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_key(&self, kid: &str) -> Option<AppleKey> {
        let keys = self.keys.read().await;
        keys.get(kid).cloned()
    }

    pub async fn refresh_keys(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://appleid.apple.com/auth/keys")
            .send()
            .await?
            .json::<KeyResponse>()
            .await?;

        let mut keys = self.keys.write().await;
        keys.clear();
        for key in response.keys {
            keys.insert(key.kid.clone(), key);
        }

        Ok(())
    }
}
