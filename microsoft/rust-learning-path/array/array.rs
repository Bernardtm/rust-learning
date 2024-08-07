fn main() {
    // Declare array, initialize all values, compiler infers length = 7
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    
    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];
    /*
    No tempo de compilação, a assinatura de matriz é definida como [T; size]:

    T é o tipo de dados para todos os elementos na matriz.
    size é um inteiro não negativo que representa o comprimento da matriz.
    A assinatura revela duas características importantes sobre matrizes:

    Cada elemento de uma matriz tem o mesmo tipo de dados. O tipo de dados nunca é alterado.
    O tamanho da matriz é fixo. O comprimento nunca é alterado.
    Apenas uma coisa sobre a matriz pode ser alterada ao longo do tempo: os valores dos elementos na matriz. 
    O tipo de dados permanece constante e o número de elementos (comprimento) permanece constante. Somente os valores podem ser alterados.
    */

    // Access elements in array
    // Days of the week
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    // Get the first day of the week
    let first = days[0];

    // Get the second day of the week
    let second = days[1];
}