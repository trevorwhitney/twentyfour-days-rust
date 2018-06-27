use std::env;

mod day2;

fn main() {
    let mut args = env::args();
    let result = args
        .nth(1)
        .ok_or("Incorrect usage: please specify a day to define which algorithm to run".to_owned())
        .and_then(|arg| {
            arg.parse::<usize>().map_err(|err| err.to_string())
        })
        .and_then(|day| {
            match day {
                1 => day1(args),
                2 => day2(args),
                _ => {
                    Err(format!("Invalid day {}", day))
                }
            }
        });

    match result {
        Ok(message) => println!("{}", message),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1)
        }
    }
}

fn day1(mut args: env::Args) -> Result<String, String> {
    args.nth(0)
        .ok_or("Please provide a number to test if prime".to_owned())
        .and_then(|arg| arg.parse::<usize>().map_err(|err| err.to_string()))
        .map(|suspect| format!("{} is prime: {}", suspect, day2::is_prime(suspect)))
        .map_err(|err| err.to_string())
}

fn day2(mut args: env::Args) -> Result<String, String> {
    args.nth(0)
        .ok_or("Please provide the number prime you'd like to find".to_owned())
        .and_then(|arg| arg.parse::<usize>().map_err(|err| err.to_string()))
        .map(|n| day2::find_prime(n))
}


