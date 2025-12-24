use std::{env::args, fs::File, io::{BufReader, Read}, vec};


fn main() {
    let args:Vec<String> = args().collect();
    println!("{}",args.len());
    if args.len() > 2{
        println!("Usage: jlox [script]");
    }else if args.len() == 1 {
        // call the run file with file path
    }else {
        run_prompt();
    }
}
fn run_prompt () {
    println!("running the prompt");
}

fn run_file (path: String) { 
    // open the file
    // read it 
    let f = File::open(path).expect("error opening file");
    let mut buf = Vec::new();
    let mut reader = BufReader::new(f);
    reader.read_to_end(&mut buf);
    println!("running the file");

    // run 
    run(&buf);
}

fn run (source: &Vec<u8>) {

}