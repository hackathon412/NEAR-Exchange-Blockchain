use crate::*;

impl Contract {
    pub fn register_ft_address(&mut self, req: RegisterToken) {
        let caller = env::predecessor_account_id();
        if self.owner_id !=caller{
            env::panic_str("not owner")
        }
        for item in &req {
            self.ft_address.insert(item.address.clone(),item.ft.clone());
        }
    }
}
