mod models;
mod services;

use models::account::Account;
use services::bank_service::BankService;

fn main() {
    let mut abhishek = Account::new("abhishek",100);
    let mut jhanvi = Account::new("jhanvi",3100);

   
    abhishek.deposit(2000);
    abhishek.withdraw(1000);
    println!("{}", abhishek.summary());
    jhanvi.withdraw(100);
    println!("{}", jhanvi.summary());

    let mut bank_service = BankService::new("CITI Bank");
    bank_service.create_account("Alice", 2000);
    bank_service.create_account("Bob", 3100);
    
    bank_service.bank.total_balance();
    bank_service.display_summary();
    let account =bank_service.get_account("Alice");
    account.withdraw(50);
    println!("{}", account.summary());

}
