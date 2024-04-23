pub struct BankAccount {
    m_customer_name: String,
    m_balance: f64,
}
impl BankAccount {
    pub fn new(customer_name: &str, balance: f64) -> BankAccount {
        BankAccount {
            m_customer_name: String::from(customer_name),
            m_balance: balance,
        }
    }

    pub fn get_customer_name(&self) -> &str {
        &self.m_customer_name[..]
    }

    pub fn get_balance(&self) -> f64 {
        self.m_balance
    }

    pub fn debit(&mut self, amount: f64) {
        if amount > self.m_balance {
            panic!("Debit amount is bigger than balance");
        }
        if amount < 0.0 {
            panic!("Debit amount is negative");
        }
        self.m_balance -= amount;
    }

    pub fn credit(&mut self, amount: f64) {
        if amount < 0.0 {
            panic!("Credit amount is negative");
        }
        self.m_balance += amount;
    }
}