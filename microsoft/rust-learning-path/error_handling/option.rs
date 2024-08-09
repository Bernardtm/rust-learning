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
/*
Você pode refinar a expressão match ainda mais para atuar de maneira diferente, dependendo dos valores dentro de uma variante Some. 
Por exemplo, você poderia enfatizar o fato de que cocos são incríveis executando o código seguinte:
O primeiro padrão na correspondência é Some(&"coconut") (observe o & antes do literal de cadeia de caracteres). 
Isso ocorre porque o fruits.get(index) retorna um Option<&&str> ou uma opção de uma referência a uma fatia de cadeia de caracteres. 
A remoção de & no padrão significaria que estamos tentando fazer a correspondência com relação a um Option<&str> 
(uma fatia de cadeia de caracteres opcional não é uma referência opcional a uma fatia de cadeia de caracteres). 
Não abordamos as referências, portanto, talvez isso não faça muito sentido no momento. 
Por enquanto, apenas lembre-se de que & está garantindo que os tipos se alinhem corretamente.
*/
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(&"coconut") => println!("Coconuts are awesome!!!"),
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}
/*
Sempre que usar a expressão match, tenha em mente as seguintes regras:

Os braços match são avaliados de cima para baixo. 
Casos específicos precisam ser definidos antes de casos genéricos ou eles nunca serão correspondidos e avaliados.
Os braços match devem cobrir cada valor possível que o tipo de entrada possa ter. 
Você receberá um erro do compilador se tentar fazer uma correspondência com uma lista de padrões não exaustiva.
*/

/*
No exemplo a seguir, a entrada para match é um valor Option<u8>. 
A expressão match só deverá executar o código se o valor de entrada for 7.
*/
let a_number: Option<u8> = Some(7);
match a_number {
    Some(7) => println!("That's my lucky number!"),
    _ => {},
}
/*
Nesse caso, gostaríamos de ignorar a variante None e todos os valores Some<u8> que não correspondem a Some(7). 
Padrões curinga são úteis para esse tipo de situação. 
Você pode adicionar o padrão de curinga _ (sublinhado) após todos os outros padrões para fazer a correspondência com qualquer outra coisa, 
e ele é usado para atender às demandas do compilador para esgotar os braços de correspondência.

Para condensar esse código, você pode usar uma expressão if let:
Um operador if let compara um padrão com uma expressão. Se a expressão corresponder ao padrão, o bloco if será executado. 
O que é legal nas expressões if let é que você não precisa de todo o código clichê de uma expressão match quando está interessado 
em um só padrão para fazer a correspondência.
*/
let a_number: Option<u8> = Some(7);
if let Some(7) = a_number {
    println!("That's my lucky number!");
}

// unwrap e expect
/*
Você pode tentar acessar o valor interno de um tipo Option diretamente usando o método unwrap. 
No entanto, tenha cuidado, pois esse método entrará em pane se a variante for um None.
*/
let gift = Some("candy");
assert_eq!(gift.unwrap(), "candy");

let empty_gift: Option<&str> = None;
assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!
//     thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:6:27

/*
O método expect faz o mesmo que unwrap, mas fornece uma mensagem de pane personalizada fornecida por seu segundo argumento:
*/
let a = Some("value");
assert_eq!(a.expect("fruits are healthy"), "value");

let b: Option<&str> = None;
b.expect("fruits are healthy"); // panics with `fruits are healthy`
//     thread 'main' panicked at 'fruits are healthy', src/main.rs:6:7


/*
Como estas funções podem entrar em pane, não é recomendável usá-las. Em vez disso, considere uma das seguintes abordagens:

Use a correspondência de padrões e trate o caso de None explicitamente.
Chame métodos semelhantes que não causam pane, como unwrap_or, que retornará um valor padrão se a variante for None ou 
o valor interno se a variante for Some(value).
assert_eq!(Some("dog").unwrap_or("cat"), "dog");
assert_eq!(None.unwrap_or("cat"), "cat");
*/