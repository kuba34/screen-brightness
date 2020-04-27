use getopts::Options;
use std::{env, fs};

const WRITE_PATH: &str = "/sys/class/backlight/radeon_bl0/brightness";
const READ_PATH: &str = "/sys/class/backlight/radeon_bl0/actual_brightness";

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("s", "set", "Set screen brightness", "<b>")
        .optflag("g", "get", "Get screen brighness")
        .optflag("h", "help", "Print this message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        },
    };

    if matches.opt_present("h") {
        let brief = format!("{}", opts.short_usage(&args[0]));
        print!("{}", opts.usage(&brief));
    }
    else if matches.opt_present("s") {
        let b: isize = matches.opt_str("s").unwrap().parse().unwrap();
        if b > 255 || b < 0 {
            eprintln!("Number out of range");
            std::process::exit(1);
        }
        fs::write(WRITE_PATH, format!("{}", b))?;
    }
    else if matches.opt_present("g") {
        let tmp = fs::read(READ_PATH)?;
        let current_val = String::from_utf8_lossy(&tmp);
        print!("Actual brightness: {}", current_val);
    }
    else {
        println!("{}", opts.short_usage(&args[0]));
        std::process::exit(1);
    }
    Ok(())
}
