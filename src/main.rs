use std::{
    env::args,
    fs::File,
    io::{self, BufReader, Read},
};

fn main() {
    let args: Vec<String> = args().collect();
    println!("{}", args.len());
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 1 {
        // reading from file panic
        run_file(&args[1]).expect("error running file");
    } else {
        run_prompt();
    }
}
fn run_prompt() {
    println!(">");
    for line in io::stdin().lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                break;
            }
            run(&l.as_bytes())
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
    println!("running the file");

    // run
    run(&buf);
    Ok(())
}

fn run(source: &[u8]) {
    // run the source code
    // we can use match and remove unsafe
    let s: &str = unsafe { str::from_utf8_unchecked(source) };
    println!("Result: {}", s); 
}
