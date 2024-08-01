fn main() {
    /* Você pode declarar uma nova variável que usa o nome de uma variável existente. 
    A nova declaração cria uma associação. No Rust, essa operação é chamada de "sombreamento" 
    porque a nova variável sombreia a variável anterior. A variável antiga ainda existe, 
    mas não é mais possível consultá-la neste escopo.
     */
    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num = shadow_num + 5; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 

    println!("The number is {}.", shadow_num);
}