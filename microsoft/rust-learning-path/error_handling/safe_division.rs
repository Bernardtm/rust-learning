#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}
/*
O tipo Result também tem os métodos unwrap e expect, que:

Retornam o valor dentro da variante Ok.
Fazer com que o programa entre em pane se a variante for um Err.
Vamos ver Result em ação. No seguinte código de exemplo, há uma implementação para uma função safe_division que retorna:

Um valor Result com uma variante Ok que transporta o resultado de uma divisão bem-sucedida.
Uma variante Err que transporta um struct DivisionByZeroError, que sinaliza uma divisão sem êxito.

A parte #[derive(Debug)] que precede o struct DivisionByZeroError é uma macro que diz ao compilador Rust para tornar o tipo imprimível para fins de depuração. 
Abordaremos esse conceito em detalhes posteriormente, no módulo sobre Características.
*/