use std::{
    env::args,
    f32::consts::E,
    fs::{self, File},
    io::{self, BufReader, Read, Write, stdout},
    iter::Scan, vec,
    collections::HashMap,
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
    let tokens = sc.scan_tokens()?;
    for t in tokens {
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
        let c = self.advance()?;
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
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.advance()?;
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
            '"' => {
                self.string()?;
            }
            '0'..'9' => {
                self.number();
            }
            _ => {
                // if char is alphabatic call identifier
                if c.is_alphabetic() {
                    self.identifier();

                    return Ok(());
                }    
                return Err(JiloxError::error(
                    self.line,
                    "unexpected character".to_string(),
                ));
            }
        }
        Ok(())
    }

    // advance will advance to the next char in the source and return the next char
    fn advance(&mut self) -> Result<char, JiloxError> {
        let result = self.source.get(self.current).copied().ok_or_else(|| JiloxError::error(self.line, "Unexpected end of source".to_string()))?;
        self.current += 1;
        Ok(result)
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

    // peak will return the char
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn string(&mut self) -> Result<(), JiloxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line+=1;
            }
            self.advance()?;
        }
        if self.is_at_end() {
            return Err(JiloxError::error(self.line, "unterminated string".to_string()));
        }
        // capture the last "
        self.advance()?;

        let value:String = self.source[self.start + 1 .. self.current - 1].iter().collect();
        self.add_token_objects(TokenType::STRING, Some(Literal::String(value)));
        Ok(())
    }

    fn is_digit (&self ,c:char) -> bool {
        return c >= '0' && c <= '9'
    }

    fn number (&mut self) {
        // 11 it can be pass with this but this will stop at . because its not digit
        while self.is_digit(self.peek()) {
            let _ = self.advance();
        }
        // but for 11.111 we need to consume that . also
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            let _ = self.advance();


            while self.is_digit(self.peek()) {
                let _ = self.advance();
            }
        }

        let value:String = self.source[self.start .. self.current].iter().collect();
        let num:f64 = value.parse().unwrap();
        self.add_token_objects(TokenType::NUMBER, Some(Literal::Number(num)));
    }

    fn peek_next (&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0' 
        }else {
            return self.source[self.current + 1];
        }
    }

    fn identifier (&mut self) {
        /* 
            1. peek till alpha numeric and alpha numeric will 
            check if it is alphabetic or is_digit 

            2.  define hashmap with keywords and check for that text in 
                map if found call addtoken with that type  
            3. else call it with identifier
        */

        while self.c_a_an(self.peek()) {
            let _ = self.advance();
        } 

        let s:String = self.source[self.start .. self.current].iter().collect();
        // TODO: where to define the map and how to get the correct type
        if let Some (tt) = Scanner::keyword_check(s.as_str()) {
            self.add_tokens(tt);
        }else {
            self.add_tokens(TokenType::IDENTIFIER);
        }
                 
    }

    fn c_a_an(&self,c:char) -> bool {
       if c.is_alphabetic() || self.is_digit(c) {
            return true;
       }else {
            return false;
        }  
    } 

    pub fn keyword_check (word:&str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "if" => Some(TokenType::IF),
            "else" => Some(TokenType::ELSE),
            "true" => Some(TokenType::TRUE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::CLASS),
            "fun" => Some(TokenType::FUN),

            "nil"=> Some(TokenType::NIL),

            "or"=> Some(TokenType::OR),

            "print" => Some(TokenType::PRINT),

            "return" => Some(TokenType::RETURN),

            "super" => Some(TokenType::SUPER),

            "this" => Some(TokenType::THIS),

            "var" => Some(TokenType::VAR),

            "while" => Some(TokenType::WHILE),

            _ => {
                None
            }
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
