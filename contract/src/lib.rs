use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Timestamp, Promise};
use near_sdk::collections::LookupMap;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    lockers: LookupMap<AccountId, Timestamp>,
    caviar: LookupMap<AccountId, u128>,
    caviar_vault: LookupMap<AccountId, u128>,
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
      caviar_vault: LookupMap::new(b"vault".to_vec()),
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
        let nemo = self.get_nemo(account_id.clone());

        if caviar > 0 {
            let rand: u8 = *env::random_seed().get(0).unwrap();
            let new_caviar = caviar - 2;
            self.set_caviar(account_id.clone(), new_caviar);
            if rand == 29 || rand == 1 || rand == 19 || rand == 5 || rand == 21 || rand == 20 {
                self.set_nemo(account_id, nemo + 10);
                return "You win 10 nemo... lucky you!".to_owned();
            } else if rand < 50 {
                self.set_caviar(account_id, caviar + 10);
                return "You win 10 caviar!".to_owned();
            } else if rand < 100 {
                self.set_nemo(account_id, nemo + 1);
                return "You win 1 nemo!".to_owned();
            } else if rand < 150 {
                self.set_caviar(account_id, caviar + 20);
                return "You win 20 caviar!".to_owned();
            } else if rand < 200 {
                self.set_caviar(account_id, caviar + 30);
                return "You win 30 caviar!".to_owned();
            } else {
                self.set_caviar(account_id, caviar + 50);
                return "You win 50 caviar!".to_owned();
            }
        }

        
        
        "No caviar - no lottery!".to_owned()
    }

    pub fn harvest_fish(&mut self, account_id: AccountId) -> u128 {
        let mut harvest: u128 = 0;
        let caviar = self.get_caviar(account_id.clone());
        harvest = harvest 
            + u128::from(self.get_nemo(account_id.clone()))
            + (u128::from(self.get_dori(account_id.clone())) * 2)
            + (u128::from(self.get_captain(account_id.clone())) * 5)
            + (u128::from(self.get_ariel(account_id.clone())) * 10);
        if harvest > 0 {
            self.set_caviar(account_id, caviar + harvest);
        }
        harvest
    }

    pub fn get_caviar_vault(&self, account_id: AccountId) -> u128 {
        match self.caviar_vault.get(&account_id) {
            Some(caviar) => caviar,
            None => 0,
        }
    }

    pub fn set_caviar_vault(&mut self, account_id: AccountId, new_caviar: u128) {
        self.caviar_vault.remove(&account_id);
        self.caviar_vault.insert(&account_id, &new_caviar);
    }

    pub fn stake_caviar(&mut self, account_id: AccountId, amount: u128) -> u128 {
        let mut staked: u128 = 0;
        let caviar = self.get_caviar(account_id.clone());
        if amount > 0 && amount <= caviar {
            self.set_caviar_vault(account_id.clone(), amount);
            self.set_caviar(account_id.clone(), caviar - amount);
            staked = amount;
        }
        staked
    }

    ///unstake all staked caviar from vault to account
    pub fn unstake_caviar(&mut self, account_id: AccountId) -> u128 {
        let mut unstaked: u128 = 0;
        let vault = self.get_caviar_vault(account_id.clone());
        if vault > 0 {
            let caviar = self.get_caviar(account_id.clone());
            self.set_caviar_vault(account_id.clone(), 0);
            self.set_caviar(account_id, caviar + vault);
            unstaked = vault;
        }
        unstaked
    }

    //100 staked = 1 harvested
    pub fn harvest_stake (&mut self, account_id: AccountId) -> u128 {
        let mut harvested: u128 = 0;
        let vault = self.get_caviar_vault(account_id.clone());
        let times = vault / 100;
        if times > 0 {
            let caviar = self.get_caviar(account_id.clone());
            self.set_caviar(account_id, caviar + times);
            harvested = times;
        }

        harvested
    }
    /// swap 500 caviar to 1 nemo
    pub fn swap_caviar_to_nemo(&mut self, account_id: AccountId) -> u16 {
        let mut swapped: u16 = 0;
        let nemo: u16 = self.get_nemo(account_id.clone());
        let caviar: u128 = self.get_caviar(account_id.clone());
        if caviar >= 500 {
            self.set_nemo(account_id.clone(), nemo + 1);
            self.set_caviar(account_id, caviar - 500);
            swapped = 1;
        }
        swapped
    }
    /// swap 10 nemo to 1 dori
    pub fn swap_nemo_to_dori(&mut self, account_id: AccountId) -> u16 {
        let mut swapped: u16 = 0;
        let nemo: u16 = self.get_nemo(account_id.clone());
        let dori: u16 = self.get_dori(account_id.clone());
        let caviar: u128 = self.get_caviar(account_id.clone());
        if nemo >= 10 && caviar >= 100 {
            self.set_nemo(account_id.clone(), nemo - 10);
            self.set_dori(account_id.clone(), dori + 1);
            self.set_caviar(account_id, caviar - 100);
            swapped = 1;
        }
        swapped
    }
    /// swap 10 dori to 1 captain
    pub fn swap_dori_to_captain(&mut self, account_id: AccountId) -> u16 {
        let mut swapped: u16 = 0;
        let dori: u16 = self.get_dori(account_id.clone());
        let captain: u16 = self.get_captain(account_id.clone());
        let caviar: u128 = self.get_caviar(account_id.clone());
        if dori >= 10 && caviar >= 100 {
            self.set_dori(account_id.clone(), dori - 10);
            self.set_captain(account_id.clone(), captain + 1);
            self.set_caviar(account_id, caviar - 100);
            swapped = 1;
        }
        swapped
    }
    /// swap 10 captain to 1 ariel
    pub fn swap_captain_to_ariel(&mut self, account_id: AccountId) -> u16 {
        let mut swapped: u16 = 0;
        let captain: u16 = self.get_captain(account_id.clone());
        let ariel: u16 = self.get_ariel(account_id.clone());
        let caviar: u128 = self.get_caviar(account_id.clone());
        if captain >= 10 && caviar >= 100 {
            self.set_captain(account_id.clone(), captain - 10);
            self.set_ariel(account_id.clone(), ariel + 1);
            self.set_caviar(account_id, caviar - 100);
            swapped = 1;
        }
        swapped
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
