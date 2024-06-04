fn eh_primo(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    // rounding up
    let limite = (n as f64).sqrt().ceil() as i32 + 1;

    for i in 2..limite {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let  numero = 75835;

    println!("O valor {} e primo? {}", numero, if eh_primo(numero) { "e primo" } else { "nao e primo" });
}
