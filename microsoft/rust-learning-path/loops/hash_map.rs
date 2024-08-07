fn main() {
    /*
    Outro tipo de coleção comum no Rust é o mapa de hash. 
    O tipo HashMap<K, V> armazena dados mapeando cada chave K com o respectivo valor V. 
    Embora os dados em um vetor sejam acessados usando um índice de número inteiro, os dados em um mapa de hash são acessados usando uma chave.

    O tipo de mapa de hash é usado em muitas linguagens de programação para itens de dados como objetos, tabelas de hash e dicionários.

    Assim como os vetores, os mapas de hash podem ser aumentados. 
    Os dados são armazenados no heap e o acesso aos itens do mapa de hash é verificado em tempo de execução.
     */

     use std::collections::HashMap;
     /*
     O comando use traz a definição HashMap da parte collections da biblioteca padrão do Rust para o escopo do nosso programa. 
     Essa sintaxe é semelhante ao que outras linguagens de programação chamam de importação.
      */
     let mut reviews: HashMap<String, String> = HashMap::new();
     /*
     Criamos um mapa de hash vazio com o método HashMap::new. 
     Declaramos a variável reviews como mutável para que possamos adicionar ou remover chaves e valores, conforme necessário. 
     Em nosso exemplo, as chaves e valores do mapa de hash usam o tipo String.
      */
     
     reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
     reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
     reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

     // Look for a specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
    /*
    Observe que a saída exibe a crítica do livro como "Some("Ótimos exemplos.".)" em vez de apenas "Ótimos exemplos". 
    Como o método get retorna um tipo Option<&Value>, o Rust encapsula o resultado da chamada de método com a notação "Some()".
     */

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
    /*
    Podemos remover as entradas de um mapa de hash usando o método .remove(). 
    Se usarmos o método get para uma chave de mapa de hash inválida, o método get retornará "None".
    */
}