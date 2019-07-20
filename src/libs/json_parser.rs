use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::Chars;
use std::str::FromStr;

type F64 = f64;
type I64 = i64;

#[derive(Debug, PartialEq, PartialOrd)]
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

    fn next_char(&mut self, trim_whitespace: bool) -> Option<char> {
        self.cursor = self.s.next();
        if trim_whitespace {
            while let Some(c) = self.cursor {
                match c {
                    '\n' | '\r' | '\t' | ' ' => self.cursor = self.s.next(),
                    _ => break,
                };
            }
        }
        self.cursor
    }

    fn parse(&mut self) -> Result<Value, String> {
        self.trim_whitespaces();
        if let Some(Ok(v)) = self.parse_literal() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_string() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_number() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_array() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_object() {
            Ok(v)
        } else {
            Err(format!("parse json error"))
        }
    }

    /// 返回之后指针指向最后一个字符
    fn get_str(&mut self, len: usize) -> Vec<char> {
        let mut vc = vec![];
        loop {
            if let Some(c) = self.cursor {
                vc.push(c);
            } else {
                break;
            }
            if len == vc.len() {
                break;
            }
            self.next_char(false);
        }
        vc
    }

    fn trim_whitespaces(&mut self) {
        while let Some(c) = self.cursor {
            match c {
                '\n' | '\r' | '\t' | ' ' => self.next_char(true),
                _ => break,
            };
        }
    }

    fn parse_literal(&mut self) -> Option<Result<Value, String>> {
        let mut literal = None;

        if let Some(c) = self.cursor {
            match c {
                't' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s == "true" {
                        literal = Some(Ok(Value::Bool(true)));
                    } else {
                        return Some(Err(format!(
                            "parse bool error: expect 'true' found {:?}",
                            s
                        )));
                    }
                }
                'f' => {
                    let s = self.get_str(5).into_iter().collect::<String>();
                    if s == "false" {
                        literal = Some(Ok(Value::Bool(false)));
                    } else {
                        return Some(Err(format!(
                            "parse bool error: expect 'false' found {:?}",
                            s
                        )));
                    }
                }
                'n' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s == "null" {
                        literal = Some(Ok(Value::Null));
                    } else {
                        return Some(Err(format!("parse error: expect 'null' found {}", s)));
                    }
                }
                _ => {}
            }
        }
        if let Some(_) = literal {
            // 确保跳出后指向下一个字符
            self.next_char(true);
        }
        literal
    }

    fn parse_number(&mut self) -> Option<Result<Value, String>> {
        let mut nv = vec![];
        while let Some(c) = self.cursor {
            match c {
                '0'..='9' | '+' | '-' | 'e' | 'E' | '.' => {
                    nv.push(c);
                    self.next_char(false);
                }
                _ => break,
            }
        }
        if nv.len() == 0 {
            return None;
        }
        // 匹配数字结束后已经指向下一个字符，所以只去除空白符即可
        self.trim_whitespaces();
        let nstring = nv.into_iter().collect::<String>();
        let nstr = &nstring[..];
        if let Some(_) = nstr.find('.') {
            Some(
                F64::from_str(nstr)
                    .map_err(|e| -> String { format!("parse f64 error: {}", e) })
                    .map(|v| -> Value { Value::Number(Number::F64(v)) }),
            )
        } else {
            Some(
                I64::from_str(nstr)
                    .map_err(|e| -> String { format!("parse i64 error: {}", e) })
                    .map(|v| -> Value { Value::Number(Number::I64(v)) }),
            )
        }
    }

    fn parse_string(&mut self) -> Option<Result<Value, String>> {
        let mut name_start = false;
        let mut nv = vec![];
        let c = self.cursor?;
        if c != '"' {
            return Some(Err(format!("unexpect character {}, expect '\"'", c)));
        }
        while let Some(c) = self.cursor {
            match c {
                '\\' => {
                    let next = self.next_char(false)?;
                    match next {
                        '"' => {
                            nv.push('"');
                        }
                        '\\' => {
                            nv.push('\\');
                        }
                        '/' => {
                            nv.push('/');
                        }
                        'b' => {
                            nv.push('\x08');
                        }
                        'f' => {
                            nv.push('\x0C');
                        }
                        'n' => {
                            nv.push('\n');
                        }
                        'r' => {
                            nv.push('\r');
                        }
                        't' => {
                            nv.push('\t');
                        }
                        'u' => {
                            self.next_char(false);
                            let s = self.get_str(4).into_iter().collect::<String>();
                            if let Ok(i) = u32::from_str_radix(&s[..], 16) {
                                if let Ok(ic) = char::try_from(i) {
                                    nv.push(ic);
                                }
                            } else {
                                return Some(Err(format!(
                                    "escape unicode characters error : {}",
                                    s
                                )));
                            }
                        }
                        _ => {
                            return Some(Err(format!("unexpected escaped character {}", next)));
                        }
                    };
                }
                '"' => {
                    if name_start {
                        // 确保跳出后指向下一个字符
                        self.next_char(true);
                        return Some(Ok(Value::String(nv.into_iter().collect::<String>())));
                    }
                    name_start = true;
                }
                other => {
                    if !name_start {
                        return Some(Err(format!("parse string error , string start not found")));
                    }
                    nv.push(other);
                }
            };
            self.next_char(false);
        }
        None
    }

    fn parse_array(&mut self) -> Option<Result<Value, String>> {
        let c = self.cursor?;
        if c != '[' {
            return Some(Err(format!(
                "unexpected array start, expect [ but found {}",
                c
            )));
        }
        let mut arr_start = false;
        let mut nv = vec![];
        while let Some(c) = self.cursor {
            match c {
                ']' => {
                    if !arr_start {
                        return Some(Err(format!("parse array error")));
                    }
                    // 确保跳出后指向下一个字符
                    self.next_char(true);
                    return Some(Ok(Value::Array(nv)));
                }
                ',' | '[' => {
                    if c == '[' {
                        arr_start = true;
                    } else {
                        if !arr_start {
                            return Some(Err(format!("parse array error")));
                        }
                    }
                    self.next_char(true);
                    if let Some(Ok(v)) = self.parse_array() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_literal() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_number() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_string() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_object() {
                        nv.push(v);
                    } else {
                        return Some(Err(format!(
                            "parse array error: unexpected {}",
                            self.cursor?
                        )));
                    }
                }
                other => {
                    return Some(Err(format!("unexpected character '{}'", other)));
                }
            }
            self.trim_whitespaces();
        }
        None
    }

    fn parse_object(&mut self) -> Option<Result<Value, String>> {
        let c = self.cursor?;
        if c != '{' {
            return Some(Err(format!(
                "parse object failed: unexpected character {}",
                c
            )));
        }

        let mut ob_start = false;
        let mut obj = HashMap::new();
        while let Some(c) = self.cursor {
            match c {
                '{' => {
                    ob_start = true;
                    self.next_char(true);
                    if let Some(Ok(Value::String(name))) = self.parse_string() {
                        let colon = self.cursor?;
                        if colon != ':' {
                            return Some(Err(format!("expect colon but found {}", colon)));
                        }
                        self.next_char(true);
                        if let Some(Ok(v)) = self.parse_object() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_array() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_literal() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_string() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_number() {
                            obj.insert(name, v);
                        } else {
                            return Some(Err(format!(
                                "parse object error, unexpected {}",
                                self.cursor?
                            )));
                        }
                    }
                }
                '}' => {
                    if !ob_start {
                        return Some(Err(format!("unexpected object close brace")));
                    }
                    // 确保跳出后指向下一个字符
                    self.next_char(true);
                    return Some(Ok(Value::Object(obj)));
                }
                ',' => {
                    self.next_char(true);
                    if let Some(Ok(Value::String(name))) = self.parse_string() {
                        let colon = self.cursor?;
                        if colon != ':' {
                            return Some(Err(format!("expect colon but found {}", colon)));
                        }
                        self.next_char(true);
                        if let Some(Ok(v)) = self.parse_object() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_array() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_literal() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_string() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_number() {
                            obj.insert(name, v);
                        } else {
                            return Some(Err(format!(
                                "parse object error, unexpected {}",
                                self.cursor?
                            )));
                        }
                    }
                }
                c => return Some(Err(format!("unexpected character {}", c))),
            }
            self.trim_whitespaces();
        }
        None
    }
}

pub fn test() {
    let data = r#"
        {
          "\u4e05" : [
            "test中国\n\u4e06",
            100,
            false
          ],
          "测试" : 100,
        }
    "#
    .to_string();
    {
        let mut chars = data.chars();
        let mut pit = ParserIter::new(&mut chars);
        println!("{:?}", pit.parse());
    }
}
