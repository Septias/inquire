use std::fmt::{Display, Formatter};

use inquire::{
    error::InquireResult, required, ui::RenderConfig, CustomType, DateSelect, MultiSelect, Select,
    Text,
};

fn main() -> InquireResult<()> {
    let _date = DateSelect::new("Date:")
        .with_render_config(RenderConfig::empty_static_ref())
        .prompt()?;

    let _category = Select::new("Category:", get_categories())
        .with_render_config(RenderConfig::empty_static_ref())
        .prompt()?;

    let _payee = Text::new("Payee:")
        .with_validator(required!("This field is required"))
        .with_suggester(&payee_suggestor)
        .with_help_message("e.g. Music Store")
        .with_page_size(5)
        .with_render_config(RenderConfig::empty_static_ref())
        .prompt()?;

    let amount: f64 = CustomType::new("Amount:")
        .with_formatter(&|i| format!("${}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("Type the amount in US dollars using a decimal point as a separator")
        .with_render_config(RenderConfig::empty_static_ref())
        .prompt()
        .unwrap();

    let _description = Text::new("Description:")
        .with_help_message("Optional notes")
        .with_render_config(RenderConfig::empty_static_ref())
        .prompt()?;

    let mut accounts = get_accounts();
    let accounts_mut = accounts.iter_mut().collect();
    let account = Select::new("Account:", accounts_mut)
        .with_render_config(RenderConfig::empty_static_ref())
        .prompt()?;
    account.balance -= amount;

    let _tags = MultiSelect::new("Tags:", get_tags())
        .with_render_config(RenderConfig::empty_static_ref())
        .prompt()?;

    println!("Your transaction has been successfully recorded.");
    println!("The balance of {} is now $311.09", account);

    Ok(())
}

/// This could be retrieved from a database, for example.
fn get_tags() -> Vec<&'static str> {
    vec![
        "august-surprise",
        "birthday-gifts",
        "cat-aurora",
        "christmas-gifts-2020",
        "dog-bob",
        "dog-russ",
        "new-zealand-jan-2020",
        "roma-oct-2021",
    ]
}

struct Account {
    name: &'static str,
    balance: f64,
}

impl Account {
    pub fn new(name: &'static str, balance: f64) -> Self {
        Self { name, balance }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

/// This could be retrieved from a database, for example.
fn get_accounts() -> Vec<Account> {
    vec![
        Account::new("401k", 1.00),
        Account::new("Cash", 10.00),
        Account::new("D40 Bank", 100.00),
        Account::new("D40 Bank Credit Card", 1000.00),
        Account::new("Digital Wallet", 100.00),
        Account::new("Established Bank", 10.00),
        Account::new("Investments Account", 1.00),
        Account::new("Meal Voucher", 10.00),
        Account::new("Mortgage", 100.00),
        Account::new("Zeus Bank Credit Card", 354.08),
    ]
}

/// This could be retrieved from a database, for example.
fn get_categories() -> Vec<&'static str> {
    vec![
        "Rent",
        "Energy",
        "Water",
        "Internet",
        "Phone",
        "Groceries",
        "Eating Out",
        "Transportation",
        "Gifts",
        "Clothes",
        "Home Appliances",
    ]
}

/// This could be faster by using smarter ways to check for matches, when dealing with larger datasets.
fn payee_suggestor(input: &str) -> Vec<String> {
    let input = input.to_lowercase();

    get_existing_payees()
        .iter()
        .filter(|p| p.to_lowercase().contains(&input))
        .take(5)
        .map(|p| String::from(*p))
        .collect()
}

/// This could be retrieved from a database, for example.
fn get_existing_payees() -> &'static [&'static str] {
    &[
        "Armstrong-Jacobs",
        "Barrows-Becker",
        "Becker PLC",
        "Bins, Fritsch and Hartmann",
        "Feil PLC",
        "Frami-Fisher",
        "Goyette Group",
        "Heathcote PLC",
        "Hilpert-Kovacek",
        "Keebler Inc",
        "Kuhn-Rippin",
        "McGlynn LLC",
        "McKenzie, Kris and Yundt",
        "Medhurst, Conroy and Will",
        "Ruecker LLC",
        "Steuber, Casper and Hermann",
        "Torphy-Boyer",
        "Volkman, Smith and Shanahan",
        "VonRueden-Rath",
        "Waelchi and Sons",
    ]
}
