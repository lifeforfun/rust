use std::error::Error;

pub fn test()
{

}

pub struct RegExp {
    pub inner: RegExpType,
}

pub enum RegExpType {
    Literal(RegExp),
    Group(RegExp),
}

pub struct RegExpResult {

}

pub struct ParseError(String);
pub struct MatchError(String);

impl RegExp {

    /// parse to an `RegExp` object
    pub fn parse(reg: String) -> Result<RegExp, ParseError> {

    }

    pub fn exec(&mut self, text: String) -> Result<RegExpResult, MatchError > {

    }
}