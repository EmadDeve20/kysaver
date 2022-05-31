use std::process;
use std::io::Write;
use std::fs;
use std::io;

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
        
        println!("{}", keylogger);
        write!(file, "{}", keylogger).unwrap_or_else(|_| {
            process::exit(1);
        });
    }
}
