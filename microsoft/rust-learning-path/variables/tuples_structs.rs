fn main() {
    // Tuples
    /* Uma tupla é um agrupamento de valores de tipos diferentes coletados em um valor composto. 
    Os valores individuais em uma tupla são chamados de elementos. 
    Os valores são especificados como uma lista separada por vírgulas colocada entre parênteses (<value>, <value>, ...).

    Uma tupla tem um comprimento fixo, que é igual ao número de elementos que ela contém. 
    Depois que uma tupla é declarada, ela não pode aumentar ou diminuir de tamanho. 
    Elementos não podem ser adicionados nem removidos. 
    O tipo de dados de uma tupla é definido pela sequência dos tipos de dados dos elementos. 
    */
    // Tuple of length 3
    let tuple_e = ('E', 5i32, true);
    /* Os elementos em uma tupla podem ser acessados pela posição do índice, começando em zero. 
    Esse processo é conhecido como indexação de tupla. Para acessar um elemento em uma tupla, usamos a sintaxe <tuple>.<index>. 
    */
    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

    // Struct
    /* Um struct é um tipo que é composto de outros tipos. 
    Os elementos em um struct são chamados de campos. 
    Assim como as tuplas, os campos em um struct podem ter tipos de dados diferentes. 
    Um benefício significativo do tipo de struct é que você pode nomear cada campo para que seja claro o que o valor significa.

    Para trabalhar com structs em um programa Rust, você primeiro define o struct por nome e especifica o tipo de dados para cada campo. 
    Em seguida, você cria uma instância do struct com outro nome. Ao declarar a instância, você fornece os valores específicos para os campos.

    O Rust dá suporte a três tipos de struct: structs clássicos, structs de tupla e structs de unidade. 
    Esses tipos de struct dão suporte a maneiras diferentes de agrupar dados e trabalhar com eles.

    - Os structs C clássicos são os mais usados. Cada campo no struct tem um nome e um tipo de dados.
     Depois que um struct clássico é definido, os campos no struct podem ser acessados usando a sintaxe <struct>.<field>.
    - Os structs de tupla são semelhantes aos structs clássicos, mas os campos deles não têm nomes. 
    Para acessar os campos em um struct de tupla, usamos a mesma sintaxe que usamos para indexar uma tupla: <tuple>.<index>. 
    Assim como nas tuplas, os valores de índice no struct de tupla começam em zero.
    - Os structs de unidade são usados com mais frequência como marcadores. 
    Descobriremos mais sobre por que os structs de unidade são úteis quando conhecermos o recurso de características(trait) do Rust.

    Ao contrário da convenção de nomenclatura que já utilizamos até agora, o nome de um tipo de struct é em maiúsculas.

    Os tipos de struct geralmente são definidos fora da função main e de outras funções no programa Rust. 
    Por esse motivo, o início da definição de struct não é recuado da margem esquerda. 
    Somente a parte interna da definição é recuada para mostrar como os dados são organizados.
    */
    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool } // Uma definição de struct clássico não termina com um ponto e vírgula.

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32); 
    /* Assim como em uma tupla, o corpo de um struct de tupla é definido dentro de parênteses (). 
    Os parênteses ficam logo após o nome do struct. 
    Não há espaço entre o nome do struct e o parêntese de abertura.
    */

    // Unit struct
    struct Unit;

    // Instantiating a struct
    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };
    // como o tipo eh String, devemos usar o String::from para criar uma nova string, se nao identificara como &str
    // O compilador sugere que podemos usar a função .to_string() para fazer a conversão.

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);


    // Enums
    /* Usamos a palavra-chave enum para criar um tipo enumerado, que pode ter qualquer combinação das variantes de enumeração. 
    Assim como os structs, as variantes de enumeração podem ter campos com nomes, campos sem nomes ou nenhum campo. 
    Os tipos de enumeração, assim como os tipos de struct, também são escritos em maiúsculas. 
    */
    enum WebEvent {
        // An enum variant can be like a unit struct without fields or data types
        WELoad, // não tem nenhum dado nem tipo de dados associado.
        // An enum variant can be like a tuple struct with data types but no named fields
        WEKeys(String, char), // tem dois campos com os tipos de dados String e char.
        // An enum variant can be like a classic struct with named fields and their data types
        WEClick { x: i64, y: i64 } // contém um struct anônimo com campos nomeados x e y e os respectivos tipos de dados (i64).
    }

    /*
    Uma forma de resolver os requisitos de variante de enumeração é definir um struct separado para cada variante na enumeração. 
    Em seguida, cada variante na enumeração usa o struct correspondente. 
    O struct contém os mesmos dados que foram mantidos pela variante de enumeração correspondente. 
    Esse estilo de definição permite que nos refiramos a cada variante lógica de maneira independente.

    O código a seguir mostra como usar esse estilo de definição alternativo. 
    Os structs são definidos para manter os dados. 
    As variantes na enumeração são definidas para se referirem aos structs.
     */
    // Define a tuple struct
    struct KeyPress(String, char);

    // Define a classic struct
    struct MouseClick { x: i64, y: i64 }

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

    // Instantiate enums
    /* Agora, vamos adicionar código para criar instâncias de nossas variantes de enumeração. 
    Para cada variante, usamos a palavra-chave let para fazer a atribuição. 
    Para acessar a variante específica na definição de enumeração, usamos a sintaxe <enum>::<variant> com dois-pontos duplos ::.
     */
    let we_load = WebEvent::WELoad(true);
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };

    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);

    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };

    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);


    // Set the Debug flag so we can check the data in the output
    // #[derive(Debug)]

    /* A sintaxe #[derive(Debug)] nos permite ver determinados valores durante a execução do código que, de outra forma, 
    não podem ser visualizados na saída padrão. Para exibir dados de depuração com a macro println!, usamos a sintaxe {:#?} 
    para formatar os dados de maneira legível. 
    */


}