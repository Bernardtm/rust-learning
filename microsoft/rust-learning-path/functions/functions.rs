fn main() {
    goodbye();
    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye_with_arg(formal);
    goodbye_with_arg(casual);

    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}

// local onde a funcao eh definida nao importa
fn goodbye() {
    println!("Goodbye.");
}

fn goodbye_with_arg(message: &str) {
    println!("\n{}", message);
}

/*
Retornar um valor
Quando uma função retorna um valor, adicionamos a sintaxe -> <type> após a lista de argumentos de função e antes da chave de abertura para o corpo da função. 
A sintaxe de seta -> indica que a função retorna um valor ao chamador. A parte <type> permite que o compilador saiba o tipo de dados do valor retornado.

No Rust, a prática comum é retornar um valor no final de uma função fazendo com que a última linha de código na função seja igual ao valor a ser retornado. 

Podemos usar a palavra-chave return em qualquer ponto na função para interromper a execução e retornar um valor ao chamador. 
Normalmente, a palavra-chave return é usada em combinação com um teste condicional.
*/
fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // Return early
        return 0;
    }
    num / 5
}