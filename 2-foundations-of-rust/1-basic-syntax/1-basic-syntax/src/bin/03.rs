fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut max = input[0];
    let mut min = input[0];
    for x in input {
        if x > max {
            max = x;
        }
        if x < min {
            min = x;
        }
    }

    println!("{} is largest and {} is smallest", max, min);
}
