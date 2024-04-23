pub mod bank;

use bank::BankAccount;

fn main() {
    let mut acc: BankAccount = BankAccount::new("Mr. Bryan Walton", 11.99);
    acc.credit(5.77);
    acc.debit(11.22);
    println!("Current balance is {}", acc.get_balance());
}
