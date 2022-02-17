use std::io;
extern crate regex;
use regex::Regex;

#[derive(Debug)]
enum Token {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Numeric(f64),
}

impl Token {
    fn calc(&self, stack: &mut Vec<f64>) -> Result<f64,String> {
        use Token::*;
        match self {
            Addition => {
                let y = stack.pop().ok_or(format!("Error: Addition y"))?;
                let x = stack.pop().ok_or(format!("Error: Addition x"))?;
                Ok(x + y)
            },
            Substraction => {
                let y = stack.pop().ok_or(format!("Error: Substraction y"))?;
                let x = stack.pop().ok_or(format!("Error: Substraction x"))?;
                Ok(x - y)
            },
            Multiplication => {
                let y = stack.pop().ok_or(format!("Error: Multiplication y"))?;
                let x = stack.pop().ok_or(format!("Error: Multiplication x"))?;
                Ok(x * y)
            },
            Division => {
                let y = stack.pop().ok_or(format!("Error: Division y"))?;
                let x = stack.pop().ok_or(format!("Error: Division x"))?;
                Ok(x / y)
            },
            Numeric(x) => Ok(*x),
        }
    }
}

fn make_token(s: &str) -> Result<Vec<Token>,String> {
    let re = Regex::new(r"[0-9\.]+").unwrap();
    let mut tokenlist = Vec::new();
    for token in s.split_whitespace() {
        let tk = match token {
            "+" => Token::Addition,
            "-" => Token::Substraction,
            "*" => Token::Multiplication,
            "/" => Token::Division,
            _ => {
                let num = re.captures(token)
                    .and_then(|x| x.at(0))
                    .and_then(|x| x.parse::<f64>().ok())
                    .ok_or(format!("Fail to parse {}", token))?;
                Token::Numeric(num)
            }
        };
        tokenlist.push(tk);
    }
    Ok(tokenlist)
}

fn rpn(tokenlist: &[Token]) -> Result<f64,String> {
    let mut stack: Vec<f64> = Vec::new();
    for token in tokenlist {
        let x = token.calc(&mut stack)?;
        stack.push(x)
    }
    stack.pop().ok_or(format!("Fail to get the last value"))
}

fn main() {
    let mut tl = String::new();
    println!("式を入力してください");
    io::stdin().read_line(&mut tl).expect("Failed to read line");
    let tl = tl.trim();

    match make_token(tl)
        .and_then(|tokenlist| rpn(&tokenlist)) {
        Ok(ans) => println!("Tokenlist = {}, Answer = {}", tl, ans),
        Err(e) => println!("Error {}", e),
    }
}
