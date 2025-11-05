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
         if let Err(err) = self.bank.remove_account(holder_name) {
            eprintln!("{}", err);
        } else {
            println!("Account '{}' removed.", holder_name);
        }
    }


    pub fn deposit(&mut self,holder_name:&str,amount:u64){
      match self.bank.get_account(holder_name){
        Some(acc) =>{
           if let Err(err) = acc.deposit(amount) {
                    eprintln!("Deposit failed for {}: {}", holder_name, err);
                } else {
                    println!("Dear {} amount {}rs Deposited to your account",holder_name,amount)

                }
        },
        None => eprintln!("No account found for {}", holder_name),
      }
    }

    pub fn withdraw(&mut self,holder_name:&str,amount:u64){
      match self.bank.get_account(holder_name){
        Some(acc) => match acc.withdraw(amount){
          Ok(_)=>println!("Dear {} amount {}rs debited from your account", holder_name, amount),
          Err(err) => eprintln!("Withdrawal failed for {}: {}", holder_name, err),
        },
        None =>eprintln!("No account found for {}", holder_name),
      }
    }
      pub fn total_balance(&self) {
        println!(
            "{} has total balance: {}rs",
            self.bank.bank_name,
            self.bank.total_balance()
        );
    }
}   