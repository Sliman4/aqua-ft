use near_sdk::{AccountId, assert_one_yocto, borsh::{self, BorshDeserialize, BorshSerialize}, near_bindgen};
use near_sdk_contract_tools::{hook::Hook, ft::*, owner::*, Owner};

#[derive(BorshSerialize, BorshDeserialize, Default, FungibleToken, Owner)]
#[fungible_token(transfer_hook = "NonTransferrableHook")]
#[near_bindgen]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn set_metadata(&mut self, metadata: FungibleTokenMetadata) {
        assert_one_yocto();
        Owner::assert_owner(self);
        Nep148Controller::set_metadata(self, &metadata);
    }

    #[private]
    #[payable]
    pub fn set_owner(&mut self, owner_id: AccountId) {
        assert_one_yocto();
        Owner::update_owner(self, Some(owner_id));
    }
}

pub struct NonTransferrableHook;

impl<C> Hook<C, Nep141Transfer<'_>> for NonTransferrableHook {
    fn hook<R>(_contract: &mut C, _args: &Nep141Transfer<'_>, _f: impl FnOnce(&mut C) -> R) -> R {
        panic!("Token is non-transferable");
    }
}
