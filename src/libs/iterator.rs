pub fn test() {
    dump("http://www.baidu.com".bytes());
    scan();
    take_while();
    half_open();
}

fn triangle(n: i32) -> i32 {
    (1..n+1).fold(0, |sum,item|sum+item)
}

use std::fmt::Debug;

fn dump<T,U>(t: T)
where
    T: IntoIterator<Item=U>,
    U: Debug
{
    for u in t {
        println!("{:?}", u);
    }
}

fn scan()
{
    let iter = (0..10)
        .scan(0, |sum, item| {
            *sum += item;
            if *sum>10 {
                None
            } else {
                Some(item*item)
            }
        });
    let collect = iter.collect::<Vec<i32>>();
    println!("iter:{:?}", collect);
    assert_eq!(collect, vec![0, 1, 4, 9, 16]);
}

fn take_while() {
    let message = "To: jimb\r\n\
        From: superego <editor@oreilly.com>\r\n
        \r\n\
        Did you get any writing done today?\r\n\
        When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().take_while(|line|!line.is_empty()) {
        println!("{}", header);
    }
}

fn half_open() {
    for i in 0.. {
        if i>900 {
            break
        }
        println!("{}", i);
    }
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'),(1,'B'),(2,'C'),(3,'D')]);
}

fn fuse() {
    struct Flaky(bool);
    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0{
                self.0=false;
                Some("totally the last item")
            } else {
                self.0=true;
                None
            }
        }
    }
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));
}