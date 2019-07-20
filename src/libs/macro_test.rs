#[macro_export(local_inner_macros)]
macro_rules! t {
    ( $($x:expr), * ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    }
}

pub fn test() {
    println!("{:?}", t![1, 2, 3]);
}
