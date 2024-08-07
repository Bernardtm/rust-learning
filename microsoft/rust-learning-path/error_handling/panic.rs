fn main() {
    /*
    Entrar em pânico é o mecanismo tratamento de erro mais simples no Rust.

    Você pode usar a macro panic! para entrar em pânico na thread atual. 
    A macro imprime uma mensagem de erro, libera recursos e fecha o programa.

    Este programa seria fechado com o código de status 101 e imprimiria a seguinte mensagem:
    thread 'main' panicked at 'Farewell!', src/main.rs:2:5
     */
    // panic!("Farewell!");
    /*
    Em termos gerais, você deve usar panic! quando um programa atinge um estado irrecuperável. 
    Um estado em que não há absolutamente nenhuma maneira de se recuperar do erro.
    O Rust emite uma pane em algumas operações, como uma divisão por zero ou uma tentativa de 
    acessar um índice que não está presente em uma matriz, um vetor ou um mapa de hash, 
    conforme mostrado no seguinte código:
     */
    let v = vec![0, 1, 2, 3];
    println!("{}", v[6]); // this will cause a panic!
}