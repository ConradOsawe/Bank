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

// Borrowing - using a reference 
fn print_account(account: &Account)  {
    println!("{:#?}", account);
    
}

fn main() {

  let  account = Account::new(647736328, String::from("John"));

  let account_ref1 = &account;
  let account_ref2 = &account;
 
   print_account(account_ref1);
   print_account(account_ref2);

  println!("{:#?}", account);

}
