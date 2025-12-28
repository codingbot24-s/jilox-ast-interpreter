use std::{
    env::args,
    f32::consts::E,
    fs::{self, File},
    io::{self, BufReader, Read, Write, stdout},
    iter::Scan,
};

struct JiloxError {
    line: usize,
    message: String,
}

impl JiloxError {
    fn error(line: usize, message: String) -> JiloxError {
        JiloxError { line, message }
    }

    fn report(&self, wher: String) {
        eprintln!(
            "[line {}  ] Error +  {} + : {}",
            self.line, wher, self.message
        )
    }
}

fn run_prompt() {
    for line in io::stdin().lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                break;
            }
            match run(l) {
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
    // read string
    let source = fs::read_to_string(path).expect("error reading file");
    // TODO: cur passing bytes we need to pass the string so scanner can use it
    match run(source) {
        Ok(()) => {}
        Err(e) => {
            e.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run(source: String) -> Result<(), JiloxError> {
    // run the source code
    // we can use match and remove unsafe
    let mut sc = Scanner::new(source);
    let token = sc.scan_tokens()?;
    for t in token {
        println!("{}", t);
    }
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
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
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
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    // TODO: return the tokens
    /*
        scan_token will check if we are at the end of source then it will
        push the eof token and fucntion will end else it will recursively call itself

    */
    fn scan_tokens(&mut self) -> Result<&Vec<Token>, JiloxError> {
        let mut had_error: Option<JiloxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(()) => {}
                Err(err) => {
                    err.report("".to_string());
                    had_error = Some(err);
                }
            }
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Some(Literal::Nil),
            self.line,
        ));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }
    // check if we are at the end of source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // will check the token type and add the token
    fn scan_token(&mut self) -> Result<(), JiloxError> {
        let c = self.advance();
        match c {
            '(' => self.add_tokens(TokenType::LEFTPAREN),
            ')' => self.add_tokens(TokenType::RIGHTPAREN),
            '{' => self.add_tokens(TokenType::LEFTBRACE),
            '}' => self.add_tokens(TokenType::RIGHTBRACE),
            ',' => self.add_tokens(TokenType::COMMA),
            '.' => self.add_tokens(TokenType::DOT),
            '-' => self.add_tokens(TokenType::MINUS),
            '+' => self.add_tokens(TokenType::PLUS),
            ';' => self.add_tokens(TokenType::SEMICOLON),
            '*' => self.add_tokens(TokenType::STAR),
            // TODO: check if this is != if false then call add token with bang type
            '!' => {
                if self.peek_char('=') {
                    self.add_tokens(TokenType::BANGEQUAL);
                } else {
                    self.add_tokens(TokenType::BANG);
                }
            }
            '=' => {
                if self.peek_char('=') {
                    self.add_tokens(TokenType::EQUALEQUAL);
                } else {
                    self.add_tokens(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.peek_char('=') {
                    self.add_tokens(TokenType::LESSEQUAL);
                } else {
                    self.add_tokens(TokenType::LESS);
                }
            }
            '>' => {
                if self.peek_char('=') {
                    self.add_tokens(TokenType::GREATEREQUAL);
                } else {
                    self.add_tokens(TokenType::GREATER);
                }
            }

            '/' => {
                if self.peek_char('/') {
                    // then its is comment we need to move further
                    println!("found comment skipiing");
                    while self.skip_comment() != '\n' && !self.is_at_end() {
                        let _ = self.advance();
                    }
                } else {
                    self.add_tokens(TokenType::SLASH);
                }
            }
            ' ' => {println!("Space skiping")} 
            '\r' => {println!("\r skiping")}
            '\t' => {println!("\t skiping")} 
            '\n' => {
                println!("line was {} ",self.line);
                self.line+=1;
                println!("line is {} ",self.line);
            }
            _ => {
                return Err(JiloxError::error(
                    self.line,
                    "unexpected character".to_string(),
                ));
            }
        }
        Ok(())
    }

    // advance will advance to the next char in the source and return the next char else will return None
    fn advance(&mut self) -> char {
        // TODO: remove unwrap it will panic
        let result = *self.source.get(self.current).expect("errorr getting next char");
        self.current+=1;
        result
    }
    // it will call the token add_token_objects with None
    fn add_tokens(&mut self, ttype: TokenType) {
        self.add_token_objects(ttype, None)
    }
    // add the tokens in Vec
    fn add_token_objects(&mut self, ttype: TokenType, object: Option<Literal>) {
        let lexeme = &self.source[self.start..self.current];
        let s: String = lexeme.into_iter().collect();
        self.tokens.push(Token::new(ttype, s, object, self.line));
    }

    /*

        check the next char if it is expected then return
        true else false
    */
    fn peek_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn skip_comment(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
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
