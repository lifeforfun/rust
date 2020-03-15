use std::error::Error;
use std::fmt;

pub fn test()
{

}

#[derive(Debug)]
pub struct RegExp {
    pub inner: RegExpType,
}

#[derive(Debug)]
pub enum RegExpType {
    /// match string literal (include wildcards)
    Literal(RegExp),
    Period(RegExp),
    Group(RegExp),
}

#[derive(Debug)]
pub struct RegExpResult<'a> {
    /// match string start offset
    pub i_s: u32,
    /// match string end offset
    pub i_e: u32,
    /// the match string
    pub s: &'a str,
    /// sub matches group
    pub sub: Vec<RegExpResult<'a>>,
}

#[derive(Debug)]
pub struct ParseError(String);
#[derive(Debug)]
pub struct MatchError(String);

impl Error for ParseError {}
impl Error for MatchError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: ", self.0)
    }
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: ", self.0)
    }
}

impl RegExp {

    /// parse to an `RegExp` object
    pub fn parse(reg: String) -> Result<RegExp, ParseError> {

    }

    pub fn exec(&mut self, text: String) -> Result<RegExpResult, MatchError > {

    }
}