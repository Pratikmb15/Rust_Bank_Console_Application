mod services;
mod models;

use services::bank_service::BankService;

fn main() {

    let mut bank_service = BankService::new("CITI Bank");

    bank_service.create_account("Alice", 1000);
    bank_service.create_account("Bob", 2000);

    bank_service.deposit("Alice", 500);
    bank_service.withdraw("Alice", 200);
    bank_service.withdraw("Alice", 5000); // Triggers error

    bank_service.total_balance();

    bank_service.delete_account("Charlie"); // Triggers error
}
