#[derive(Debug)]
#[allow(dead_code)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn summary(&self) -> String {
        format!("Account {} has a balance of {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, "Alice".to_string());
    account.deposit(100);
    account.withdraw(50);

    let mut account2 = Account::new(2, "Bob".to_string());
    account2.deposit(200);
    account2.withdraw(100);

    bank.add_account(account);
    bank.add_account(account2);

    println!("{:#?}", bank.summary());
    println!("{:#?}", bank.total_balance());
}
