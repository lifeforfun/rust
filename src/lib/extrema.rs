#[derive(Debug)]
pub struct Extrema<'e> {
    greatest: &'e i32,
    least: &'e i32
}

pub fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least {least = &slice[i];}
        if slice[i]>*greatest {greatest=&slice[i];}
    }
    Extrema{greatest, least}
}