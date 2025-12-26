use core::fmt;
use std::{
    env::args,
    fmt::write,
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

#[derive(Debug)]
enum TokenType {
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    IDENTIFIER,
    STRING,
    NUMBER,

    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}


#[derive(Debug)]
enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, litreal: Option<Literal>, line: usize) -> Token {
        Token {
            token_type: ttype,
            lexeme: lexeme,
            literal: litreal,
            line: line,
        }
    }
    
}

// TODO: Impl scanner
struct Scanner {
    source:String,
    start:usize,
    current:usize,
    line:usize,
    tokens:Vec<Token>,
}

impl Scanner {
    // TODO: error move of tokens
    fn scan_tokens (&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_tokens();
        } 
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), Some(Literal::Nil), self.line));
    }
    fn is_at_end (&self) -> bool {
        self.current >= self.source.len()    
    }
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
