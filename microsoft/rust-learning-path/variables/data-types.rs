fn main() {
    // Inteiros
    let number: u8 = 14; // inteiro de 8 bits sem sinal
    let number: i8 = 14; // inteiro de 8 bits com sinal
    let number: u16 = 14; // inteiro de 16 bits sem sinal
    let number: i16 = 14; // inteiro de 16 bits com sinal
    let number: u32 = 14; // inteiro de 32 bits sem sinal
    let number: i32 = 14; // inteiro de 32 bits com sinal
    let number: u64 = 14; // inteiro de 64 bits sem sinal
    let number: i64 = 14; // inteiro de 64 bits com sinal
    let number: u128 = 14; // inteiro de 128 bits sem sinal
    let number: i128 = 14; // inteiro de 128 bits com sinal
    let number: usize = 14; // inteiro de x bits sem sinal, depende da arquitetura
    let number: isize = 14; // inteiro de x bits com sinal, depende da arquitetura
    // por padrao, os inteiros sao do tipo i32

    // Ponto flutuante
    let number: f32 = 14.0; // ponto flutuante de 32 bits
    let number: f64 = 14.0; // ponto flutuante de 64 bits
    // por padrao, os pontos flutuantes sao do tipo f64

    // Operations Examples
    // Addition, Subtraction, and Multiplication
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
    /*
       Quando chamamos a macro println, adicionamos o sufixo de tipo de dados a cada número literal 
       para informar o tipo de dados ao Rust. A sintaxe 1u32 informa ao compilador que o valor é o 
       número 1 e para interpretar o valor como um inteiro de 32 bits sem sinal.

        Se não fornecermos anotações de tipo, o Rust tentará inferir o tipo do contexto. 
        Quando o contexto é ambíguo, ele atribui o tipo i32 (um inteiro com sinal de 32 bits) por padrão.
     */

    // Booleans - true or false
    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);  // false

    // Text - Char
    // Todos os tipos de texto são representações UTF-8 válidas.
    let letter = 'A';
    let smiley_face = '😊';
    /* Algumas linguagens tratam os tipos char como inteiros sem sinal de 8 bits, 
    que é equivalente do tipo u8 do Rust. O tipo char no Rust contém pontos de código Unicode, 
    mas não usam a codificação UTF-8. Um char no Rust é um inteiro de 21 bits que é preenchido para ter 32 bits. 
    O char contém o valor de ponto de código simples diretamente.
    */

    // Text - Cadeias de caracteres
    /*
    O tipo str, também conhecido como uma fatia de cadeia de caracteres, 
    é uma exibição dos dados de cadeia de caracteres.
     Na maioria das vezes, nos referimos a esses tipos usando a sintaxe de estilo de referência que precede o tipo com o E comercial &str. 
     Abordaremos as referências nos módulos a seguir. Por enquanto, você pode considerar &str como um ponteiro para dados de cadeia de caracteres imutáveis. 
     Os literais de cadeia de caracteres são do tipo &str.

    Embora usar literais de cadeia de caracteres em exemplos introdutórios do Rust seja conveniente, 
    eles não são adequados para todas as situações em que queremos usar um texto. 
    Nem toda cadeia de caracteres pode ser conhecida no tempo de compilação. 
    Um exemplo disso ocorre quando um usuário interage com um programa durante o runtime e envia um texto por meio de um terminal.

    Para esses cenários, o Rust tem um segundo tipo de cadeia de caracteres chamado String. Esse tipo é alocado no heap. 
    Quando você usa o tipo String, o comprimento da cadeia de caracteres (número de caracteres) não precisa ser conhecido antes da compilação do código.

    Observação

    Se você estiver familiarizado com uma linguagem com coleta de lixo, talvez esteja se perguntando por que o Rust tem dois tipos de cadeia de caracteres.
    1 Cadeias de caracteres são tipos de dados extremamente complexos. A maioria das linguagens usa os respectivos coletores de lixo para mitigar essa complexidade.
    Enquanto linguagem do sistema, o Rust expõe parte da complexidade inerente das cadeias de caracteres. Com essa complexidade adicional, 
    vem uma quantidade muito refinada de controle sobre como a memória é usada no programa.

    1 _Na verdade, o Rust tem mais que dois tipos de cadeia de caracteres. 
    Neste módulo, abordamos apenas os tipos String e &str. 
    Na documentação do Rust, saiba mais sobre os tipos de cadeia de caracteres oferecidos.

    Não vamos ter uma ideia completa da diferença entre String e &str até conhecermos a propriedade e o sistema de empréstimo do Rust. 
    Até lá, você pode pensar em dados de tipo String como dados de texto que podem mudar conforme o programa é executado. 
    As referências &str são exibições imutáveis nos dados de texto que não se alteram conforme o programa é executado.
    */
    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);

}