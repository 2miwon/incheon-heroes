use dto::nft::Nft;

use crate::config;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub struct NftMetadata {
    pub name: String,
    pub image: String,
    pub description: String,
    #[serde(default)]
    pub attributes: Vec<NftAttribute>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: String,
}

impl NftMetadata {
    pub async fn fetch(id: i64) -> dto::Result<Self> {
        let base_url = config::get().nft_metadata_base_url;
        let url = format!("{base_url}/{id}.json");
        rest_api::get(&url).await
    }

    pub async fn fetch_by_uri(url: &str) -> dto::Result<Self> {
        rest_api::get(&url).await
    }

    pub fn character(&self) -> String {
        self.attributes
            .iter()
            .find(|attr| attr.trait_type == "Character")
            .map(|attr| attr.value.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

impl From<Nft> for NftMetadata {
    fn from(value: Nft) -> Self {
        Self {
            name: value.metadata.name,
            image: value.metadata.image,
            description: value.metadata.description,
            attributes: vec![],
        }
    }
}
