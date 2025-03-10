#[derive(Debug)]
struct Account {
    balance: i32,
    account_number: u32,
    holder: String,
}

impl Account {
    fn new (account_number: u32, holder: String) -> Self {
        Account {
            account_number,
            holder,
            balance: 0,
         }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
  }
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
  let bank = Bank::new();
  let account = Account::new(647736328, String::from("John"));

//   println!("{:#?}", bank);
  print_account(account);
  print_account(account);
//   println!("{:#?}", account);
}
