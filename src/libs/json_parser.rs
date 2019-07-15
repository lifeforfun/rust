use std::collections::HashMap;
use std::str::Chars;

type F64 = f64;
type I64 = i64;

#[derive(Debug)]
enum Number {
    I64(I64),
    F64(F64),
}

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Debug)]
struct ParserIter<'a> {
    cursor: Option<char>,
    s: &'a mut Chars<'a>,
}

impl<'a> ParserIter<'a> {
    fn new(code: &'a mut Chars<'a>) -> ParserIter<'a> {
        ParserIter {
            cursor: code.next(),
            s: code,
        }
    }

    fn get_str(&mut self, len: usize) -> Vec<char>{
        let mut i = 1;
        let mut vc     = vec![];
        loop {
            if len<i {
                break;
            }
            if let Some(c) = self.cursor {
                vc.push(c);
            } else {
                return vc;
            }
            self.next();
            i += 1;
        }
        vc
    }

    fn trim_whitespaces(&mut self) {
        while let Some(c) = self.cursor {
            match c {
                '\n' | '\r' | '\t' | ' ' => self.next(),
                _ => return,
            };
        }
    }

    fn parse_literal(&mut self) -> Option<Result<Value, String>>{
        if let Some(c) = self.cursor {
            match c {
                't' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s=="true" {
                        return Some(Ok(Value::Bool(true)));
                    }
                    return Some(Err(format!("parse bool error: expect 'true' found {:?}", s)));
                },
                'f' => {
                    let s = self.get_str(5).into_iter().collect::<String>();
                    if s=="false" {
                        return Some(Ok(Value::Bool(false)));
                    }
                    return Some(Err(format!("parse bool error: expect 'false' found {:?}", s)));
                },
                'n' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s=="null" {
                        return Some(Ok(Value::Null));
                    }
                    return Some(Err(format!("parse error: expect 'null' found {}", s)));
                },
                _ => return None,
            }
        }
        None
    }

    fn parse_number(&mut self) -> Option<Result<Value::Number, String>>{
        let mut nv = vec![];
        let mut is_float = false;
        while let Some(c) = self.cursor {
            match c {
                _ => break,
            }
        }
        None
    }

}

impl <'a>Iterator for ParserIter<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.cursor = self.s.next();
        self.cursor
    }
}

pub fn test()
{
    let data = r#"
        null
    "#.to_string();
    {
        let mut chars = data.chars();
        let mut pit = ParserIter::new(&mut chars);
        pit.trim_whitespaces();
        println!("{:?}", pit.parse_literal());
    }
}