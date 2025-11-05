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
 
 pub fn deposit(&mut self,amount:u64)->Result<(),String>{
     if amount == 0 {
            return Err("Deposit amount must be greater than zero.".to_string());
        }
        self.balance += amount;
        Ok(())
   }
  
  pub fn withdraw(&mut self,amount:u64)->Result<(),String>{
     if amount == 0 {
            return Err("Withdrawal amount must be greater than zero.".to_string());
        } 
     if self.balance>=amount{
        self.balance-=amount;
        Ok(())
    }
    else {
            return Err(format!(
                "Dear {}, insufficient balance. Cannot withdraw {}rs.",
                self.holder_name, amount
            ));
  }
}
  pub fn summary(&self) -> String {
        format!("Account Holder: {}, Balance: {}rs", self.holder_name, self.balance)
    }
}
