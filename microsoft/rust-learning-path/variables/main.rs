fn main() {
    let a_number; // variable imutavel por padrao
    a_number = 10; // atribuicao de valor
    println!("The value of a_number is: {}", a_number);
    // a_number = 15; da erro, pois a variavel e imutavel
    let mut a_mutable_number = 10; // variavel mutavel
    println!("The value of a_mutable_number is: {}", a_mutable_number);
    a_mutable_number = 15; // atribuicao de valor
    println!("The value of a_mutable_number is: {}", a_mutable_number);
}