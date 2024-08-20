/*
O Rust fornece a enumeração Result<T, E> para retornar e propagar erros. 
Por convenção, a variante Ok(T) representa êxito e contém um valor e a variante Err(E) representa um erro e contém um valor de erro.

A enumeração Result<T, E> é definida como:
*/
enum Result<T, E> {
    Ok(T),  // A value T was obtained.
    Err(E), // An error of type E was encountered instead.
}
/*
O tipo Result também tem os métodos unwrap e expect, que:

Retornam o valor dentro da variante Ok.
Fazer com que o programa entre em pane se a variante for um Err.

*/