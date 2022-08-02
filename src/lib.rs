use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,log, near_bindgen, AccountId, Balance, EpochHeight};
use std::collections::HashMap;
use near_sdk::env::predecessor_account_id; 
use near_sdk::collections::Vector;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pet {
    pub name: String,
    pub pet_id: u8,
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            name: String::from("Mushu"),
            pet_id: 0,
        }
    }
}

/// Voting contract for unlocking transfers. Once the majority of the stake holders agree to
/// unlock transfer, the time will be recorded and the voting ends.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PetShopContract {
    /// How much each validator votes
    pub pet_id: u8,
    pub pet_owner_map: HashMap<AccountId, Pet>,
    pub pet_id_map: HashMap<u8, String>,
    pub pet_vec: Vector<String>,
    pub adopters: Vector<AccountId>
}

impl Default for PetShopContract {
    fn default() -> Self {
        env::panic_str("Pet Shop Contract contract should be initialized before usage")
    }
}

#[near_bindgen]
impl PetShopContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        PetShopContract {
            pet_id: 0,
            pet_owner_map: HashMap::new(),
            pet_id_map: HashMap::new(),
            pet_vec: Vector::new(vec![]),
            adopters: Vector::new(vec![]),
        }
    }


    pub fn adopt(&mut self, pet_name: &mut String) -> String {
        let new_pet = Pet {
            name: pet_name.to_string(),
            pet_id: self.pet_id,
        };
        let account_id = predecessor_account_id(); 
        self.pet_vec.push(&pet_name.to_string());
        self.adopters.push(&account_id);
        self.pet_owner_map.insert(account_id, new_pet);
        self.pet_id_map.insert(self.pet_id, pet_name.to_string());
        self.pet_id += 1; 
        log!("{} has been adopted!!", pet_name);
        return pet_name.to_string();
    }

    pub fn get_pet(&mut self, id: &u8) -> String {
        let s = String::from("No Name");
        let p = self.pet_id_map.get(id).unwrap_or(&s);
        return p.to_string();
    }

    pub fn get_owner(&mut self, id: u64) -> String {
        let s = String::from("No Name");
        let p = self.pet_vec.get(id).unwrap_or(s);
        return p.to_string();
    }


    pub fn list_pet(&mut self) -> String {
        let account_id = predecessor_account_id(); 
        let s = String::from("No Name");
        let new_pet = Pet {
            name: s,
            pet_id: self.pet_id,
        };
        let p = self.pet_owner_map.get(&account_id).unwrap_or(&new_pet);
        let r = &p.name;
        return r.to_string();
    }


    pub fn list_owner(&mut self, id: u64) -> AccountId {
        let s: AccountId = "alice.near".parse().unwrap();
        let p = self.adopters.get(id).unwrap_or(s);
        return p;
    }
    

    pub fn clear(&mut self) {
        self.pet_id = 0;
        self.pet_owner_map =  HashMap::new();
        self.pet_id_map = HashMap::new();
        self.pet_vec = Vector::new(vec![]); 
    }

}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    #[test]
    fn test_contract_initialization() {
        let mut contract = PetShopContract::new();
        let id = contract.pet_id; 
        assert_eq!(id, 0);
    }
}
