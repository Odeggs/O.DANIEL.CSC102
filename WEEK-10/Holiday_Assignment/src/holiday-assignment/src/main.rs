
//Odega daniel holiday Assignment
use std::fs::File;
use std::io::Write;

struct Company {
    name: String,
    username: String,
    password: String,
    date_founded: String,
    assets: f64,
    liabilities: f64,
    percentage_leverages: Vec<f64>,
}

fn is_valid_username(username: &str) -> bool {
    let min_len = 3;
    let max_len = 8;
username.len() >= min_len 
    && username.len() <= max_len 
    && username.chars().all(|c| c.is_ascii_alphanumeric()) 
    && username.chars().all(|c| c.is_lowercase())

}
fn is_valid_password(password: &str) -> bool {
    password.len() >= 8 && password.chars().all(|c| c.is_lowercase() && c.is_ascii_alphanumeric())
}

fn calculate_multiple(company: &Company) -> Vec<f64> {
    if company.liabilities < 10_000_000.0 {
        let multiplier = 0.05; // 5% as a decimal
        company.percentage_leverages.iter().map(|x| x * multiplier).collect()
    } else {
        Vec::new()
    }
}

fn main() {
    let companies = vec![
        Company {
            name: "Cadbury Nigeria Plc".to_string(),
            username: "cadb".to_string(),
            password: "password".to_string(),
            date_founded: "1965".to_string(),
            assets: 15000000.0,
            liabilities: 5500000.0,
            percentage_leverages: vec![0.1, 0.2, 0.3],
        },
        Company {
            name: "Champion Breweries Plc".to_string(),
            username: "cham".to_string(),
            password: "cham77".to_string(),
            date_founded: "1974".to_string(),
            assets: 25000000.0,
            liabilities: 8000000.0,
            percentage_leverages: vec![0.4, 0.5, 0.6],
        },
        Company {
            name: "Dangote Sugar Refinery".to_string(),
            username: "dang".to_string(),
            password: "password".to_string(),
            date_founded: "1970".to_string(),
            assets: 18000000.0,
            liabilities: 10000000.0,
            percentage_leverages: vec![0.7, 0.8, 0.9],
        },
        Company {
            name: "Flour Mills Nigeria Plc".to_string(),
            username: "flour".to_string(),
            password: "password".to_string(),
            date_founded: "1960".to_string(),
            assets: 32000000.0,
            liabilities: 4000000.0,
            percentage_leverages: vec![0.10, 0.11, 0.12],
        },
        Company {
            name: "Nestle Nigeria Plc".to_string(),
            username: "nestle".to_string(),
            password: "password".to_string(),
            date_founded: "1961".to_string(),
            assets: 8000000.0,
            liabilities: 1500000.0,
            percentage_leverages: vec![0.13, 0.14, 0.15],
        },
        Company {
            name: "Unilever Nigeria Plc".to_string(),
            username: "Unilever".to_string(),
            password: "password".to_string(),
            date_founded: "1923".to_string(),
            assets: 37000000.0,
            liabilities: 11000000.0,
            percentage_leverages: vec![0.16, 0.17, 0.18],
        },
        Company {
            name: "Honeywell Nigeria Plc".to_string(),
            username: "Honeywell".to_string(),
            password: "password".to_string(),
            date_founded: "1906".to_string(),
            assets: 34000000.0,
            liabilities: 9000000.0,
            percentage_leverages: vec![0.19, 0.20, 0.21],
        },
        Company {
            name: "Nigeria Breweries Plc".to_string(),
            username: "brewery".to_string(),
            password: "password".to_string(),
            date_founded: "1946".to_string(),
            assets: 30000000.0,
            liabilities: 12000000.0,
            percentage_leverages: vec![0.22, 0.23, 0.24],
        },
    ];

    let valid_companies: Vec<&Company> = companies
        .iter()
        .filter(|company| is_valid_username(&company.username) && is_valid_password(&company.password))
        .collect();

    let mut file = File::create("companies.txt").unwrap();
    println!("Done");

    for company in valid_companies {
        writeln!(
            &mut file,
            "Name: {}\nUsername: {}\nPassword: {}\nDate Founded: {}\nAssets: {}\nLiabilities: {}\nPercentage Leverages: {:?}",
            company.name, company.username, company.password, company.date_founded, company.assets, company.liabilities, company.percentage_leverages
        ).unwrap();

        if company.assets > 20_000_000.0 {
            let multiples = calculate_multiple(company);
            for (i, multiple) in multiples.iter().enumerate() {
                writeln!(&mut file, "Multiple {}: {}", i + 1, multiple).unwrap();
            }
        }
    }
}