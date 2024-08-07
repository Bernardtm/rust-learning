fn main() {
    /*
    O Rust oferece três expressões de loop para fazer um programa repetir um bloco de código:

    loop: repetir, a menos que ocorra uma parada manual.
    while: repetir enquanto uma condição permanecer verdadeira.
    for: repetir para todos os valores em uma coleção.
     */
    loop {
        println!("We loop forever!");
    }

    /*
    A expressão loop cria um loop infinito. Essa palavra-chave nos permite repetir as ações no corpo da expressão continuamente. As ações se repetem até que tenhamos uma ação direta para fazer o loop parar.

    O exemplo a seguir imprime o texto "We loop forever!" e ele não é interrompido por si só. A ação println! continua se repetindo.
    Quando você usa uma expressão loop, a única maneira de interromper o loop é se você, como o programador, intervir diretamente. 
    Você pode adicionar um código específico para fazer o loop parar ou inserir uma instrução de teclado, como Ctrl + C, para interromper a execução do programa.

    A maneira mais comum de interromper uma expressão loop é usando a palavra-chave break para definir um ponto de interrupção:
     */
    loop {
        // Keep printing, printing, printing...
        println!("We loop forever!");
        // On the other hand, maybe we should stop!
        break;                            
    }

    /*
    Quando o programa encontra a palavra-chave break, ele para de executar as ações no corpo da expressão loop e continua na próxima instrução de código.

    A palavra-chave break revela um recurso especial da expressão loop. Usando a palavra-chave break, 
    você pode parar de repetir as ações no corpo da expressão e retornar um valor no ponto de interrupção.

    O seguinte exemplo mostra como podemos usar a palavra-chave break em uma expressão loop para retornar também um valor:
    
     */
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);
    /*
    O corpo da expressão loop pode ter mais de um ponto de interrupção. 
    Quando a expressão tem vários pontos de interrupção, cada ponto de interrupção precisa retornar um valor do mesmo tipo. 
    Todos os valores precisam ser de tipo inteiro, de cadeia de caracteres ou booliano e assim por diante. 
    Quando um ponto de interrupção não retorna um valor explicitamente, o programa interpreta o resultado da expressão como uma tupla vazia, ().
     */

    // While
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }

    // For
    /*
    O loop for usa um iterador para processar uma coleção de itens. O loop repete as ações no corpo da expressão para cada item na coleção. Esse tipo de repetição de loop é chamado de iteração. Quando todas as iterações são concluídas, o loop é interrompido.

    No Rust, podemos iterar em qualquer tipo de coleção, como matriz, vetor ou mapa de hash. 
    O Rust usa um iterador para percorrer cada item da coleção, do primeiro até o último.

    O loop for usa uma variável temporária como o iterador. 
    A variável é implicitamente declarada no início da expressão de loop e o valor atual é definido com cada iteração.

    No código a seguir, a coleção é a matriz big_birds e o iterador é nomeado bird.
     */
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds
    /*
    Acessamos os itens na coleção usando o método iter(). 
    A expressão for associa o valor atual do iterador ao resultado do método iter(). 
    No corpo da expressão, podemos trabalhar com o valor do iterador.
     */
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    /*
    Outra maneira fácil de criar um iterador é usar a notação de intervalo a..b. 
    O iterador começa com o valor a e continua até b em passos de um, mas não usa o valor b.
     */
    for number in 0..5 {
        println!("{}", number * 2);
    }
    
}