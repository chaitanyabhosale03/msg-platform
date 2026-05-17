use crate::crypto::KeyPair;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub client_id: String,
    pub public_key: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub is_active: bool,
    // TODO: Add ratchet state
    // TODO: Add device fingerprint verification
}

impl Session {
    pub fn new(client_id: String, public_key: Vec<u8>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            client_id,
            public_key,
            created_at: now,
            last_activity: now,
            is_active: true,
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }

    pub fn is_expired(&self, timeout_secs: i64) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.last_activity)
            .num_seconds();
        elapsed > timeout_secs
    }
}
