use crate::*;
pub type TokenId = String;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
  pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
  pub spec: String,
  pub name: String,
  pub symbol: String,
  pub icon: Option<String>,
  pub base_uri: Option<String>,
  pub reference: Option<String>,
  pub reference_hash: Option<Base64VecU8>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]

pub struct TokenMetaData {
  pub title: Option<String>,
  pub description: Option<String>,
  pub media: Option<String>,
  pub media_hash: Option<Base64VecU8>,
  pub copies: Option<u64>,
  pub issued_at: Option<u64>,
  pub expires_at: Option<u64>,
  pub starts_at: Option<u64>,
  pub updated_at: Option<u64>,
  pub extra: Option<String>,
  pub reference: Option<String>,
  pub reference_hash: Option<Base64VecU8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
  pub owner_id: AccountId,
  pub approved_account_ids: HashMap<AccountId, u64>,
  pub next_approval_id: u64,
  pub royalty: HashMap<AccountId, u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub metadata: TokenMetaData,
  pub approved_account_ids: HashMap<AccountId, u64>,
  pub royalty: HashMap<AccountId, u32>,
}

pub trait NonfungibleTokenMetadata {
  fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonfungibleTokenMetadata for Contract {
  fn nft_metadata(&self) -> NFTContractMetadata {
    self.metadata.get().unwrap()
  }
}