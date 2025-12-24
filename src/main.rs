use std::env::args;


fn main() {
    let args:Vec<String> = args().collect();
    println!("{}",args.len());
    if args.len() > 2{
        println!("Usage: jlox [script]");
    }else if args.len() == 1 {
        run_file();
    }else {
        run_prompt();
    }
}
fn run_prompt () {
    println!("running the prompt");
}

fn run_file () { 
     println!("running the file");
}