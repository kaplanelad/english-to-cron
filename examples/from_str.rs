use std::str::FromStr;

fn main() {
    let texts = vec![
        "every 15 seconds",
        "every minute",
        "every day at 4:00 pm",
        "at 10:00 am",
        "Run at midnight on the 1st and 15th of the month",
        "on Sunday at 12:00",
    ];

    for text in texts {
        match english_to_cron::Cron::from_str(text) {
            Ok(res) => println!("{text}: {res}"),
            Err(e) => eprintln!("Error parsing '{text}': {e}"),
        }
    }
}
