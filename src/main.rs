use std::{
    env::args,
    fs::File,
    io::{self, BufReader, Read},
};

struct JiloxError {
    line: usize,
    message: String,
}

impl JiloxError {
    fn error(&self) {
        self.report("".to_string());
    }

    fn report(&self, wher: String) {
        eprintln!(
            "[line + {}  ] Error +  {} + : {}",
            self.line, wher, self.message
        )
    }
}

fn run_prompt() {
    println!(">");
    for line in io::stdin().lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                break;
            }
            match run(&l.as_bytes()) {
                Ok(()) => {}
                Err(e) => {
                    e.report("".to_string());
                }
            }
        } else {
            break;
        }
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let f = File::open(path).expect("error opening file");
    let mut buf = Vec::new();
    let mut reader = BufReader::new(f);
    reader.read_to_end(&mut buf).expect("error reading file");

    // run
    match run(&buf) {
        Ok(()) => {}
        Err(e) => {
            e.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run(source: &[u8]) -> Result<(), JiloxError> {
    // run the source code
    // we can use match and remove unsafe
    let s: &str = unsafe { str::from_utf8_unchecked(source) };
    println!("Result: {}", s);
    Ok(())
}


fn main() {
    let args: Vec<String> = args().collect();
    println!("{}", args.len());
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 1 {
        run_file(&args[1]).expect("error running file");
    } else {
        run_prompt();
    }
}


