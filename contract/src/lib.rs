use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Timestamp, Promise};
use near_sdk::collections::LookupMap;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    lockers: LookupMap<AccountId, Timestamp>,
    caviar: LookupMap<AccountId, u128>,
    nemo: LookupMap<AccountId, u16>,
    dori: LookupMap<AccountId, u16>,
    captain: LookupMap<AccountId, u16>,
    ariel: LookupMap<AccountId, u16>,
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      lockers: LookupMap::new(b"lockers".to_vec()),
      caviar: LookupMap::new(b"caviar".to_vec()),
      nemo: LookupMap::new(b"nemo".to_vec()),
      dori: LookupMap::new(b"dori".to_vec()),
      captain: LookupMap::new(b"captain".to_vec()),
      ariel: LookupMap::new(b"ariel".to_vec()),
    }
  }
}

#[near_bindgen]
impl Contract {
    ///lock 1 near in contract  to start game
    #[payable]
    pub fn init_locker(&mut self) {
        let account_id = env::signer_account_id();
        let timestamp: Timestamp = Timestamp::default();
        let amount: u128 = 1_000_000_000_000_000_000_000_000;

        if env::attached_deposit() == amount {
            self.lockers.insert(&account_id, &timestamp);
            self.get_start_pack(account_id);
        } else {
            env::panic(b"wrong deposit amount or insufficient funds");
        }
    }

    pub fn get_start_pack(&mut self, account_id: AccountId) {
        self.caviar.insert(&account_id, &365);
        self.nemo.insert(&account_id, &1);
    }

    ///current user has locker and can play
    pub fn has_locker(&self, account_id: AccountId) -> bool {
        match self.lockers.get(&account_id) {
            Some(_) => true,
            None => false,
        }
    }
    ///current user can claim his 1 near locked back to wallet
    pub fn can_unlock(&self, account_id: AccountId) -> bool {
        if self.has_locker(account_id) {
            return true;
        }
        //todo: must check timestamp + (365 * epoch) * 2)
        false
    }

    ///withdraw(and/or unlock) 1 near from contract to user wallet
    pub fn unlock(&self) {
        let amount: u128 = 1_000_000_000_000_000_000_000_000;
        let account_id = env::signer_account_id();
        if self.can_unlock(account_id.clone()) {
            Promise::new(account_id).transfer(amount);
        }
    }

    pub fn get_caviar(&self, account_id: AccountId) -> u128 {
        match self.caviar.get(&account_id) {
            Some(caviar) => caviar,
            None => 0,
        }
    }

    pub fn set_caviar(&mut self, account_id: AccountId, new_caviar: u128) {
        self.caviar.remove(&account_id);
        self.caviar.insert(&account_id, &new_caviar);
    }

    pub fn get_nemo(&self, account_id: AccountId) -> u16 {
        match self.nemo.get(&account_id) {
            Some(nemo) => nemo,
            None => 0,
        }
    }

    pub fn set_nemo(&mut self, account_id: AccountId, new_nemo: u16) {
        self.nemo.remove(&account_id);
        self.nemo.insert(&account_id, &new_nemo);
    }

    pub fn get_dori(&self, account_id: AccountId) -> u16 {
        match self.dori.get(&account_id) {
            Some(dori) => dori,
            None => 0,
        }
    }

    pub fn set_dori(&mut self, account_id: AccountId, new_dori: u16) {
        self.dori.remove(&account_id);
        self.dori.insert(&account_id, &new_dori);
    }

    pub fn get_captain(&self, account_id: AccountId) -> u16 {
        match self.captain.get(&account_id) {
            Some(captain) => captain,
            None => 0,
        }
    }

    pub fn set_captain(&mut self, account_id: AccountId, new_captain: u16) {
        self.captain.remove(&account_id);
        self.captain.insert(&account_id, &new_captain);
    }

    pub fn get_ariel(&self, account_id: AccountId) -> u16 {
        match self.ariel.get(&account_id) {
            Some(ariel) => ariel,
            None => 0,
        }
    }

    pub fn set_ariel(&mut self, account_id: AccountId, new_ariel: u16) {
        self.ariel.remove(&account_id);
        self.ariel.insert(&account_id, &new_ariel);
    }

    pub fn get_random(&mut self, account_id: AccountId) -> String {   
        let caviar = self.get_caviar(account_id.clone());

        if caviar > 0 {
            let rand: u8 = *env::random_seed().get(0).unwrap();
            let new_caviar = caviar - 1;
            self.set_caviar(account_id.clone(), new_caviar);
            if rand < 50 {
                self.set_caviar(account_id, caviar + 10);
                return "You win 10 caviar".to_owned();
            } else if rand < 100 {
                return "You loose... good luck next time!".to_owned();
            } else if rand < 150 {
                self.set_caviar(account_id, caviar + 20);
                return "You win 20 caviar".to_owned();
            } else if rand < 200 {
                self.set_caviar(account_id, caviar + 30);
                return "You win 30 caviar".to_owned();
            } else {
                self.set_caviar(account_id, caviar + 50);
                return "You win 50 caviar".to_owned();
            }
        }

        
        
        "No caviar - no lottery!".to_owned()
    }
}

/*
 *todo: inline tests
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

  
}
