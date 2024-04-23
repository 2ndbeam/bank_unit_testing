#[allow(unused_imports)]
pub mod bank;
use bank::BankAccount;
#[cfg(test)]
mod tests {
    use super::*;
    
    // This function compares floats and returns true if their difference is less or equal to permissible delta
    fn diff_less_than_delta(first: f64, second: f64, delta: f64) -> bool {
        (first - second).abs() <= delta
    }

    #[test]
    fn test_debit() {
        // Arrange
        let start_balance: f64 = 11.99;
        let debit_amount: f64 = 4.55;
        let expected: f64 = 7.44;
        let mut account: BankAccount = BankAccount::new("Mr. Bryan Walton", start_balance);
        
        //Act
        account.debit(debit_amount);

        //Assert
        let actual: f64 = account.get_balance();
        assert!(diff_less_than_delta(expected, actual, 0.001));
    }

    #[test]
    fn test_credit() {
        // Arrange
        let start_balance: f64 = 11.99;
        let credit_amount: f64 = 4.01;
        let expected: f64 = 16.0;
        let mut account: BankAccount = BankAccount::new("Mr. Bryan Walton", start_balance);
        
        // Act
        account.credit(credit_amount);

        // Assert
        let actual: f64 = account.get_balance();
        assert!(diff_less_than_delta(expected, actual, 0.001));
    }

    #[test]
    fn test_is_name_valid() {
        // Arrange
        let name: &str = "Mr. Bryan Walton";
        let expected: &str = "Mr. Bryan Walton";
        let account: BankAccount = BankAccount::new(name, 0.0);

        // Act
        let actual: &str = account.get_customer_name();

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "Debit amount is bigger than balance")]
    fn cant_take_bigger_debit() {
        let start_balance: f64 = 11.99;
        let debit_amount: f64 = 13.99;
        let mut account: BankAccount = BankAccount::new("Mr. Bryan Walton", start_balance);
        
        account.debit(debit_amount);
    }

    #[test]
    #[should_panic(expected = "Debit amount is negative")]
    fn cant_take_negative_debit() {
        let start_balance: f64 = 11.99;
        let debit_amount: f64 = -5.0;
        let mut account: BankAccount = BankAccount::new("Mr. Bryan Walton", start_balance);
        
        account.debit(debit_amount);
    }

    #[test]
    #[should_panic(expected = "Credit amount is negative")]
    fn cant_take_negative_credit() {
        let start_balance: f64 = 11.99;
        let credit_amount: f64 = -5.0;
        let mut account: BankAccount = BankAccount::new("Mr. Bryan Walton", start_balance);
        
        account.credit(credit_amount);
    }
}