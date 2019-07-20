use failure::Error;
use std::str::FromStr;
/// 给定两个**非空**链表来表示两个非负整数，数字以**反序**存储，链表里每个数字表示一个位的值
/// 两个整数相加返回一个新的链表
/// 例：
///     输入：(2,4,3) + (5,6,4)
///     输出：(7,0,8)
///     表示：342+465 = 807
use structopt::StructOpt;

type Foo<T> = Vec<T>;

#[derive(Debug, Fail)]
enum ParseNumberError {
    #[fail(display = "number out of range: {}", name)]
    OutOfRange { name: String },
}

fn parse_numbers(s: &str) -> Result<Vec<i8>, Error> {
    let mut v: Vec<i8> = vec![];
    for x in s.split(',').into_iter() {
        let n = i8::from_str(x).unwrap();
        if n < i8::from(0) || n > i8::from(9) {
            return Err(ParseNumberError::OutOfRange {
                name: format!("{}", n),
            }
            .into());
        }
        v.push(n);
    }
    Ok(v)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "add_two_numbers")]
struct Opts {
    /// numbers like `1,2,3`
    #[structopt(
        short = "x",
        long = "number1",
        value_name = "NUMBER LIST",
        required = true,
        parse(try_from_str = "parse_numbers")
    )]
    numbers1: Foo<i8>,
    /// numbers like `1,2,3`
    #[structopt(
        short = "y",
        long = "number2",
        value_name = "NUMBER LIST",
        required = true,
        parse(try_from_str = "parse_numbers")
    )]
    numbers2: Foo<i8>,
}

pub fn test() {
    let args: Opts = Opts::from_args();
    let Opts { numbers1, numbers2 } = args;
    println!("{:?}|{:?}", numbers1, numbers2);
}
