use super::account::Account;

pub struct Bank{
    bank_name: String,
    accounts: Vec<Account>
}

impl Bank{
    pub fn new(bank_name:&str)-> Self{
       Bank{
        bank_name:bank_name.to_string(),
        accounts:vec![]
       }
    }

    pub fn add_account(&mut self,account: Account){
     self.accounts.push(account)
    }

    pub fn remove_account(&mut self,holder_name:&str){
    let original_length = self.accounts.len();
    self.accounts.retain(|account|account.holder_name != holder_name);

    if self.accounts.len() < original_length{
            println!("Account of '{}' removed from bank.", holder_name);
        } else {
            println!("No account found for '{}'.", holder_name);
        }
    }
    pub fn total_balance(&self)->u64{
       let total_Balance= self.accounts.iter().map(|account|account.balance).sum();
       println!("{} has Total Balance of {}rs",self.bank_name,total_Balance);
       total_Balance
    }

    pub fn summary(&self)-> Vec<String>{
        self.accounts.iter().map(|account|account.summary()).collect::<Vec<String>>()

    }

    pub fn get_account(&mut self,holder_name:&str)->&mut Account{
       self.accounts.iter_mut().find(|a| a.holder_name == holder_name ).expect("Account not found !") 
    }
    


}