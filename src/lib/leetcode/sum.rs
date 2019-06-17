/// 题目：给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
///
/// 示例: 给定 nums = [2, 7, 11, 15], target = 9 因为 nums[0] + nums[1] = 2 + 7 = 9 所以返回 [0, 1]

use structopt::StructOpt;
use std::num::ParseIntError;

type Foo<T> = Vec<T>;

fn parse_str_vec(s: &str) -> Result<Vec<i32>, ParseIntError>
{
    let mut v:Vec<i32> = vec![];
    for x in s.split(',').into_iter() {
        v.push(i32::from_str_radix(x.trim(), 10)?);
    }
    Ok(v)
}

/// sum two numbers of a array to match target number
#[derive(Debug, StructOpt)]
#[structopt(name = "sum")]
struct Opts {
    /// numbers like `1,2,3`
    #[structopt(short="n", long="numbers", value_name="NUMBER LIST", required=true, parse(try_from_str="parse_str_vec"))]
    numbers: Foo<i32>,
    /// target number
    #[structopt(short="t", long="target", value_name="NUMBER", required=true)]
    target: i32,
}

pub fn test()
{
    let args:Opts = Opts::from_args();
    for (i, v) in args.numbers.iter().enumerate() {
        println!("{:?}", v);
    }
}