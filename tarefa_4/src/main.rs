use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Erro ao ler number");
    let number = convert_to_int(&number);
    for i in 1..=10 {
        println!("{} x {} = {}", number, i, number * i);
    }
}
