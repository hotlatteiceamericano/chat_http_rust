use std::time::Duration;

use mongodb::{IndexModel, bson};
use mongodb::options::IndexOptions;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Otp {
    pub email: String,
    pub code_hash: String,
    pub created_at: bson::DateTime,
    #[serde(skip)]
    plain_otp: String,
}

impl Otp {
    pub fn new(email: String) -> Self {
        use rand::Rng;
        let plain_otp: u32 = rand::thread_rng().gen_range(100_000..999_999);
        let plain_otp = plain_otp.to_string();
        let code_hash = hash_otp(&plain_otp);
        Self {
            email,
            code_hash,
            created_at: bson::DateTime::now(),
            plain_otp,
        }
    }

    pub fn plain_otp(&self) -> &str {
        &self.plain_otp
    }

    pub async fn create_ttl_index(db: &mongodb::Database) -> anyhow::Result<()> {
        let otps_collection = db.collection::<Otp>("otps");
        let ttl_index = IndexModel::builder()
            .keys(bson::doc! { "created_at": 1 })
            .options(
                IndexOptions::builder()
                    .expire_after(Duration::from_secs(600))
                    .build(),
            )
            .build();
        otps_collection.create_index(ttl_index).await?;
        tracing::info!("TTL index on otps collection created");
        Ok(())
    }
}

pub fn hash_otp(otp: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(otp.as_bytes());
    hex::encode(hasher.finalize())
}
