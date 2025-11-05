use crate::models::{account::Account,bank::Bank};

pub struct BankService{
     pub bank:Bank,
}

impl BankService{
    pub fn new(bank_name:&str)->Self{
     BankService{
        bank:Bank::new(bank_name)
        }
    }

    pub fn create_account(&mut self,holder_name: &str, balance: u64){
        let account = Account::new(holder_name, balance);
        self.bank.add_account(account);
    }

    pub fn display_summary(&self){
      for summary in self.bank.summary(){
        println!("{}",summary);
      }
    }

    pub fn delete_account(&mut self,holder_name:&str){
     self.bank.remove_account(holder_name);
    }

    pub fn get_account(&mut self,holder_name:&str)->&mut Account{
      self.bank.get_account(holder_name)
    }
}   