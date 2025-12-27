use core::fmt;
use std::{
    env::args,
    fmt::write,
    fs::File,
    io::{self, BufReader, Read, SeekFrom},
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
    print!(">");
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
    source:Vec<char>,
    start:usize,
    current:usize,
    line:usize,
    tokens:Vec<Token>,
}

impl Scanner {


    pub fn new (source:String) -> Self {
        Scanner {
            source:source.chars().collect(),
            start:0,
            current:0,
            line:1,
            tokens:Vec::new(), 
        }
    }

    // TODO: return the tokens
    /*  
        scan_token will check if we are at the end of source then it will 
        push the eof token and fucntion will end else it will recursively call itself

    */
    fn scan_tokens (&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_tokens();
        } 
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), Some(Literal::Nil), self.line));
    }
    // check if we are at the end of source 
    fn is_at_end (&self) -> bool {
        self.current >= self.source.len()    
    } 

    // will check the token type and add the token 
    fn scan_token (&mut self) {
        let c = self.advance(); 
            match c {
                ')' => {}
                '(' => {}
                '{' => {}
                '}' => {}
                ',' => {}
                '.' => {}
                '-' => {}
                '+' => {}
                ';' => {}
                '*' => {}

                _ => {
                    unreachable!("unknown token");
                }
            }
        
    }

    // advance will advance our cursor to the next char in the source and return the next char else will return None
    fn advance (&mut self) -> char {
        let result = *self.source.get(self.current).unwrap(); 
        self.current+=1;
        result
    }
    // it will call the token add_token_objects with None 
    fn add_tokens (&mut self,ttype:TokenType) {
        self.add_token_objects(ttype,None)
    }
    // add the tokens in Vec 
    fn add_token_objects (&mut self, ttype:TokenType,object:Option<Literal>) {
        let lexeme = &self.source[self.start .. self.current];
        let s:String = lexeme.into_iter().collect();
        self.tokens.push(Token::new(ttype, s, object, self.line));
    }
    
}

fn main() {
    
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]).expect("error running file");
    } else {
        run_prompt();
    }
}
