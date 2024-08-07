fn main() {
    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes); 
    /*
    Assim como acontece com matrizes, os vetores armazenam diversos valores com o mesmo tipo de dados. 
    Diferente de matrizes, o tamanho ou o comprimento de um vetor pode crescer ou reduzir a qualquer momento.

    Uma forma comum de declarar e inicializar um vetor é com a macro vec!. 
    Essa macro também aceita a mesma sintaxe que o construtor de matriz.

    Neste exemplo, usamos a sintaxe de ponto de interrogação de dois pontos {:?} com a macro println!. 
    O Rust não sabe como exibir um vetor de inteiros. 
    Se tentarmos exibir as informações de vetor sem usar formatação especial, 
    o compilador emitirá um erro. Usamos chaves vazias {} para ajudar a exibir os valores de vetor.
     */

    /*
     Os vetores também podem ser criados usando o método Vec::new(). 
     Esse método de criação de vetor nos permite adicionar e remover valores no final do vetor. 
     Para dar suporte a esse comportamento, declaramos a variável de vetor como mutável com a palavra-chave mut. 

     Quando criamos um vetor com o método Vec::new(), podemos adicionar e remover valores no final do vetor.

     Para adicionar um valor ao final do vetor, usamos o método push(<value>).

     Depois que o tipo de um vetor é definido como um tipo concreto, somente os valores desse tipo específico podem ser adicionados ao vetor. 
     Se tentarmos adicionar um valor de um tipo diferente, o compilador retornará um erro.
     */
    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit); 

    // Para remover o valor no final do vetor, usamos o método pop().
    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit); 

    /* Os vetores dão suporte à indexação da mesma maneira que as matrizes. 
    Podemos acessar valores de elemento no vetor usando um índice. 
    O primeiro elemento está no índice 0 e o último elemento está no comprimento do vetor, 1.
    */
    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);  

    // Como os valores de vetor são mutáveis, podemos alterar um valor em vigor acessando o valor do elemento com o índice:
    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);  

    /*
    Observar valores de índice fora de limites
    Assim como acontece com matrizes, não podemos acessar um elemento em um vetor com um índice que não está no intervalo permitido. 
    Esse tipo de expressão para uma matriz faz o compilador retornar um erro. 
    Para vetores, a compilação passa, mas o programa entra em um estado de pane irrecuperável na expressão e interrompe a execução do programa.

    Para nosso vetor de exemplo que tem três elementos, o que acontece se tentarmos acessar o elemento no índice 10?
     */
    // Access vector with out-of-bounds index
    // let beyond = index_vec[10];
    // println!("{}", beyond);
}