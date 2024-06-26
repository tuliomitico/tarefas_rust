fn maior_numero(vetor: &[i32]) -> i32 {
    let mut maior = vetor[0];
    
    if vetor.len() == 1 {
        return maior;
    }

    for i in vetor {
        if i > &maior {
            maior = *i;
        }
    }

    return maior;
}


fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", maior_numero(&vec));
}
