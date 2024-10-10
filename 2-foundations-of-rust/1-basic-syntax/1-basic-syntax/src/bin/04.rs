fn main() {
    for n in [10, 20, 30, 40] {
        let mut mult = 0;
        if n < 25 {
            mult = n * 4
        } else {
            mult = n * 3;
        };
        println!("{}", mult);
    }
}
