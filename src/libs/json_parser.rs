use std::collections::HashMap;
use std::str::Chars;
use std::str::FromStr;
use std::convert::TryFrom;

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

        let mut literal = None;

        if let Some(c) = self.cursor {
            match c {
                't' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s=="true" {
                        literal = Some(Ok(Value::Bool(true)));
                    } else {
                        return Some(Err(format!("parse bool error: expect 'true' found {:?}", s)));
                    }
                },
                'f' => {
                    let s = self.get_str(5).into_iter().collect::<String>();
                    if s=="false" {
                        literal = Some(Ok(Value::Bool(false)));
                    } else {
                        return Some(Err(format!("parse bool error: expect 'false' found {:?}", s)));
                    }
                },
                'n' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s=="null" {
                        literal = Some(Ok(Value::Null));
                    } else {
                        return Some(Err(format!("parse error: expect 'null' found {}", s)));
                    }
                },
                _ => {},
            }
        }
        // 确保返回之前向后移动游标
        self.next();
        literal
    }

    fn parse_number(&mut self) -> Option<Result<Value, String>>{

        let mut nv = vec![];
        while let Some(c) = self.cursor {
            match c {
                '0'..='9'|'+'|'-'|'e'|'E'|'.' => {
                    nv.push(c);
                    self.next();
                },
                _ => break,
            }
        }
        if nv.len()==0 {
            return None;
        }
        // 确保返回之前向后移动游标
        self.next();
        let nstring = nv.into_iter().collect::<String>();
        let nstr = &nstring[..];
        if let Some(_) = nstr.find('.') {
            Some(F64::from_str(nstr).map_err(|e| -> String {
                format!("parse f64 error: {}", e)
            }).map(|v| -> Value {
                Value::Number(Number::F64(v))
            }))
        } else {
            Some(I64::from_str(nstr).map_err(|e| -> String {
                format!("parse i64 error: {}", e)
            }).map(|v| -> Value {
                Value::Number(Number::I64(v))
            }))
        }
    }

    fn parse_string(&mut self) -> Option<Result<Value, String>> {

        let mut name_start = false;
        let mut nv = vec![];
        let c = self.cursor.unwrap();
        if c!='"' {
            return Some(Err(format!("unexpect character {}, expect '\"'", c)));
        }
        while let Some(c) = self.cursor {
            match c {
                '\\' => {
                    let next = self.next().unwrap();
                    match next {
                        '"' => {
                            nv.push('"');
                        },
                        '\\' => {
                            nv.push('\\');
                        },
                        '/' => {
                            nv.push('/');
                        },
                        'b' => {
                            nv.push('\x08');
                        },
                        'f' => {
                            nv.push('\x0C');
                        },
                        'n' => {
                            nv.push('\n');
                        },
                        'r' => {
                            nv.push('\r');
                        },
                        't' => {
                            nv.push('\t');
                        },
                        'u' => {
                            let s = self.get_str(4).into_iter().collect::<String>();
                            if let Ok(i) = u32::from_str_radix(&s[..], 16) {
                                if let Ok(ic)  = char::try_from(i) {
                                    nv.push(ic);
                                }
                            } else {
                                return Some(Err(format!("escape unicode characters error : {}", s)));
                            }
                        },
                        _ => {
                            return Some(Err(format!("unexpected escaped character {}", next)));
                        },
                    };
                    self.next();
                },
                '"' => {
                    if name_start {
                        // 确保返回之前向后移动游标
                        self.next();
                        return Some(Ok(Value::String(nv.into_iter().collect::<String>())));
                    }
                    name_start = true;
                },
                other => {
                    nv.push(other);
                },
            };
            self.next();
        }
        None
    }

    fn parse_array(&mut self) -> Option<Result<Vec<Value>, String>>{
        let mut arr_start = false;
        let mut nv: Vec<Value> = vec![];
        while let Some(c) = self.cursor {
            match c {
                ']' => {
                    if !arr_start {
                        return Some(Err(format!("parse array error")));
                    }
                    // 确保返回之前向后移动游标
                    self.next();
                    return Some(Ok(nv));
                },
                ',' | '[' => {
                    if c == '[' {
                        arr_start = true;
                    } else {
                        if !arr_start {
                            return Some(Err(format!("parse array error")));
                        }
                    }
                    self.next();
                    self.trim_whitespaces();
                    if let Some(Ok(v)) = self.parse_literal() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_number() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_string() {
                        nv.push(v);
                    } else {
                        // TODO match object
                        return Some(Err(format!("parse array error: {}", self.cursor.unwrap())));
                    }
                },
                other => {
                    return Some(Err(format!("unexpected character '{}'", other)));
                },
            }
            self.trim_whitespaces();
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
        [
        "test中国\nfdsfs",
        "test"
        ]
    "#.to_string();
    {
        let mut chars = data.chars();
        let mut pit = ParserIter::new(&mut chars);
        pit.trim_whitespaces();
        println!("{:?}", pit.parse_array());
    }
}