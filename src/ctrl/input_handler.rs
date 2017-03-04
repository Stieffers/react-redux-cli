use std::io;
use std::io::Write;
use std::path::Path;
use ansi_term::Colour::*;

pub fn verify_dir(msg: String, current_dur: String) -> String {
    print!("{} {}? ({}/{}) ", msg, Blue.italic().paint(current_dur.clone()), Green.paint("Y"), Red.paint("n"));
    match io::stdout().flush() {
        Ok(res) => res,
        Err(e) => panic!(e)
    }
    let input = get_line();
    if input.eq("n") || input.eq("N") {
        return get_dir();
    } else {
        return current_dur;
    }
}

pub fn get_dir() -> String {
    let mut count = 1;
    loop {
        print!("{}", White.bold().paint("Enter a valid directory: "));
        match io::stdout().flush() {
            Ok(res) => res,
            Err(e) => panic!(e)
        }
        let input = get_line();
        let path: &Path = Path::new(&input);
        if path.is_dir() {
            return input.clone();
        }

        print!("{}", Red.paint("Invalid directory specified.\n"));

        count += 1;

        if count > 3 {
            panic!("Invalid directory entered too many times. Aborting.");
        }
    }
}

pub fn get_line() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    return String::from(input.trim());
}