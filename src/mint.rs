use crate::*;

#[near_bindgen]
impl Contract {
  #[payable]
  pub fn nft_mint(
    &mut self, 
    token_id: TokenId, 
    metadata: TokenMetaData, 
    receiver_id: AccountId,
    perpetual_royalties: Option<HashMap<AccountId, u32>>,
  ) {
    let initial_storage_usage = env::storage_usage();

    let mut royalty = HashMap::new();

    if let Some(perpetual_royalties) = perpetual_royalties {
      assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

      for (account, amount) in perpetual_royalties {
        royalty.insert(account, amount);
      }
    }

    let token = Token {
      owner_id: receiver_id,
      approved_account_ids: Default::default(),
      next_approval_id: 0,
      royalty,
    };

    assert!(
      self.tokens_by_id.insert(&token_id, &token).is_none(),
      "Token already exists"
    );

    self.token_metadata_by_id.insert(&token_id, &metadata);

    self.internal_add_token_to_owner(&token.owner_id, &token_id);

    let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

    refund_deposit(required_storage_in_bytes);
  }
}