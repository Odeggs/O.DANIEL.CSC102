use std::fs::File;
use std::io::{self, Write};

struct Company {
    name: String,
    founded: String,
    assets: u32,
    liabilities: u32,
    percentage_leverages: Vec<u32>,
}

impl Company {
    fn new(name: &str, founded: &str, assets: u32, liabilities: u32) -> Self {
        Company {
            name: name.to_string(),
            founded: founded.to_string(),
            assets,
            liabilities,
            percentage_leverages: Vec::new(),
        }
    }
}


fn main() {
    let mut companies = Vec::new();

    // Input username and password
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let username = prompt_user_input();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = prompt_user_input();

    // Validate username and password
    if validate_username(&username) && validate_password(&password) {
        
        let company_data = [
            ("Cadbury Nigeria Plc",     "1965", 15000000, 5500000),
            ("Champion Breweries Plc",  "1974", 25000000, 8000000),
            ("Dangote Sugar Refinery",  "1970", 18000000, 10000000),
            ("Flour Mills Nigeria Plc", "1960", 32000000, 4000000),
            ("Nestle Nigeria Plc",      "1961", 8000000, 15000000),
            ("Unilever Nigeria Plc",    "1923", 37000000, 11000000),
            ("Honeywell Nigeria Plc",   "1906", 34000000, 9000000),
            ("Nigeria Breweries Plc",   "1946",  30000000, 12000000),

        ];

        for &(name, founded, assets, liabilities) in &company_data {
            println!("Enter data for {}", name);

            let mut company = Company::new(name, founded, assets, liabilities);

            if assets > 20_000_000 {
                input_leverages(&mut company);
                save_leverages_to_file(&company);
            }

            if liabilities < 10_000_000 {
                calculate_5_percent(&mut company);
            }

            companies.push(company);
        }

        // Saved companies to a file for those that are reading
        save_companies_to_file(&companies);
    } else {
        println!("Invalid username or password. Exiting.");
    }
}



fn validate_username(username: &str) -> bool {
    username.len() >= 3
    
}

fn validate_password(password: &str) -> bool {
   
    password.len() >= 8
}


fn prompt_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn input_asset(company: &mut Company) {
    println!("Enter the number of percentage leverages used by the company:");
    let number_of_leverages: u32 = prompt_user_input().parse().unwrap();

 let ue = assets - liabilities
 let number_of_leverages = ue/assets
    }
}

fn save_companies_to_file(companies: &[Company]) {
    let mut file = File::create("companies.txt").unwrap();

    for company in companies {
        writeln!(file, "{}|{}|{}|{}|{:?}", company.name, company.founded, company.assets, company.liabilities, company.percentage_leverages).unwrap();
    }
}

fn save_leverages_to_file(company: &Company) {
    let mut file = File::create(format!("{}_leverages.txt", company.name)).unwrap();

    for &leverage in &company.percentage_leverages {
        writeln!(file, "{}", leverage).unwrap();
    }
}

fn calculate_5_percent(company: &mut Company) {
    for leverage in &mut company.percentage_leverages {
        *leverage = (*leverage * 5) / 100;
    }
}

 println!("the next code");




 


