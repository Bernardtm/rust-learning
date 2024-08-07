/*
A biblioteca padrão do Rust fornece uma enumeração Option<T> a ser usada quando a ausência de um valor é uma possibilidade. 
Option<T> é amplamente usado no código Rust. É útil para trabalhar com valores que podem existir ou que podem estar vazios.

Em muitas outras linguagens, a ausência de um valor seria modelada usando null ou nil, 
mas o Rust não usa null fora do código que é interoperado com outras linguagens. 
O Rust será explícito quando um valor for opcional. 
Em muitas linguagens, uma função que usa um String pode realmente usar um String ou null. 
No Rust, essa mesma função só pode usar os Strings reais. 
Se você quiser modelar uma cadeia de caracteres opcional no Rust, precisará encapsulá-la explicitamente em um tipo Option: Option<String>.
*/

enum Option<T> {
    None,     // The value doesn't exist
    Some(T),  // The value exists
}
/*
A parte <T> da declaração da enumeração Option<T> declara que o tipo T é genérico e será associado à variante Some da enumeração Option.

Conforme discutido nas seções anteriores, None e Some não são tipos, mas variantes do tipo Option<T>, 
o que significa, entre outras coisas, que as funções não podem receber Some ou None como argumentos, mas apenas Option<T>.

Na unidade anterior, mencionamos que tentar acessar um índice inexistente de um vetor causaria panic no programa, 
mas você poderia evitar isso usando o método Vec::get, que retorna um tipo Option em vez de entrar em pane. 
Se o valor existir em um índice especificado, ele será encapsulado na variante Option::Some(value). 

Se o índice estiver fora dos limites, ele retornará um valor de Option::None em vez disso.
*/

let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

// pick the first item:
let first = fruits.get(0);
println!("{:?}", first);

// pick the third item:
let third = fruits.get(2);
println!("{:?}", third);

// pick the 99th item, which is non-existent:
let non_existent = fruits.get(99);
println!("{:?}", non_existent);


/*
Correspondência de padrões
Há um operador poderoso no Rust chamado match. 
Você pode usá-lo para controlar o fluxo do seu programa fornecendo padrões. 
Quando match encontra um padrão correspondente, ele executa o código fornecido com esse padrão.
*/
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}