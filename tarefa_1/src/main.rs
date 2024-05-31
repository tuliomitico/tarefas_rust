fn count() {
    for i in 0..=10 {
        println!("{}", i);
    }
}

fn count_down() {
    let mut i = 10;
    while i > 0 {
        println!("{}", i);
        i -= 1;
    }
}


fn main() {
    count();
    count_down();
}
