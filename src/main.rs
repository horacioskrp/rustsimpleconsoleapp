use std::{env, fmt::Display, process, str::FromStr};

#[derive(Debug)]
enum ParseMoneyError {
    InvalideInput,
    CannotParse,
}

struct Money {
    amount: f32,
    currency: String,
}

impl Money {
    fn new(amount: f32, currency: String) -> Self {
        Self { amount, currency }
    }
}

impl FromStr for Money {
    type Err = ParseMoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segment: Vec<&str> = s.split_whitespace().collect();
        match segment[..] {
            [amount, currency] => amount
                .parse::<f32>()
                .map(|x| Money::new(x, currency.to_owned()))
                .map_err(|_| ParseMoneyError::CannotParse),
            _ => return Err(ParseMoneyError::InvalideInput),
        }
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Amount {}, currency: {}", self.amount, self.currency)
    }
}

fn main() {
    let Some(user_input) = env::args().nth(1) else {
        eprint!("Invalid input");
        process::exit(1);
    };

    match user_input.parse::<Money>() {
        Ok(money) => println!("{}", money),
        Err(e) => {
            eprint!("Error: {:?}", e);
            process::exit(1);
        }
    }
}
