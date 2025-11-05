#[derive(Debug)]

pub struct Account{
    pub holder_name:String,
    pub balance:u64
}

impl Account{
 pub fn new(holder_name:&str,balance:u64)-> Self{
     Account{
     holder_name:holder_name.to_string(),
     balance:balance
    } }
 
 pub fn deposit(&mut self,amount:u64){
     self.balance+=amount;
     println!("Dear {} amount {}rs Deposited to your account",self.holder_name,amount)
   }
  
  pub fn withdraw(&mut self,amount:u64){
    if self.balance>=amount{
        self.balance-=amount;
        println!("Dear {} amount {}rs debited from your account",self.holder_name,amount)
    }
    else {
            println!(
                "Dear {}, insufficient balance. Cannot withdraw {}rs.",
                self.holder_name, amount
            );
  }
}
  pub fn summary(&self) -> String {
        format!("Account Holder: {}, Balance: {}rs", self.holder_name, self.balance)
    }
}
