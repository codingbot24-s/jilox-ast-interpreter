
fn main() {
    let v:Vec<String> = vec![];
    start(&v);    
}

fn start(args: &Vec<String>) {
    if args.len() >  1 {
        println!("Usage: jlox [script]");
    }else if  args.len() == 1 {
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