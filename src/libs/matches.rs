pub fn test() {
    let p = match 10 {
        0 => format!(""),
        1 => format!("A rabbit is nosing around in the clover"),
        n => format!("There are {} rabbits hopping about in the meadow", n),
    };
    println!("{}", p);
}
