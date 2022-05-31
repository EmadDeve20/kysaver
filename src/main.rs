use std::fs;
use std::io;
use std::io::Write;
use std::process;
use chrono::Timelike;
use chrono::offset::Local;

fn main() {
    let mut file = fs::File::create("key.log").unwrap_or_else(|err| {
        eprint!("can not open the file: {}", err);
        process::exit(1);
    });

    loop {
        let mut keylogger = String::new();

        io::stdin().read_line(&mut keylogger).unwrap_or_else(|_| {
            process::exit(1);
        });

        let now =  Local::now();
        let now = format!("\t\t\t{}:{}:{}\n", now.hour(), now.minute(), now.second());
        
        keylogger.pop();
        keylogger.push_str(&now);

        // println!("{:#?}", now);

        write!(file, "{}", keylogger).unwrap_or_else(|_| {
            process::exit(1);
        });
    }
}
