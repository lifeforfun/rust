trait Sth {
    fn foo(&self) -> &Self;
}

#[derive(Debug)]
struct StructSth {
    element: u8,
}

impl Sth for StructSth {
    fn foo(&self) -> &Self {
        &self
    }
}

pub fn test() {
    let sth = &StructSth { element: 1 };
    println!("{:?}", sth.foo())
}
