use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeedPolicy {
    None,
    Fixed { seed_id: String, bytes_hex: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FloatPolicy {
    Q16_16RoundHalfEven,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonSettings {
    pub float_policy: FloatPolicy,
}

impl Default for CanonSettings {
    fn default() -> Self {
        Self {
            float_policy: FloatPolicy::Q16_16RoundHalfEven,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunDescriptor {
    pub spec_version: String,
    pub engine_version: String,
    pub model_pack_version: String,
    pub seed_policy: SeedPolicy,
    pub io_policy: Vec<String>,
    pub normalization_profile: String,
    #[serde(default)]
    pub canon: CanonSettings,
}
