// OBJETIVO DO EXERCÍCIO:
// Atualmente, temos duas funções separadas: `odd_filter` (para filtrar números ímpares)
// e `even_filter` (para filtrar números pares).
// Seu objetivo é refatorar este código para usar uma ÚNICA função genérica chamada `filter`.
// Esta nova função `filter` deverá ser capaz de filtrar elementos de um vetor
// com base em um critério fornecido por uma closure.

// PASSOS PARA COMPLETAR O EXERCÍCIO:

// 1. DEFINA A NOVA FUNÇÃO `filter`:
//    - Nome da função: `filter`
//    - Parâmetros:
//        a. `v: &Vec<i32>` (uma referência a um vetor de inteiros)
//        b. `f: T` (onde `T` é um tipo genérico que representa uma closure)
//    - Restrição de Trait (Trait Bound) para `T`:
//        A closure `f` deve aceitar um `i32` como argumento e retornar um `bool`.
//    - Corpo da função:
//        - Crie um novo vetor vazio `result: Vec<i32>`.
//        - Itere sobre cada elemento `i` no vetor de entrada `v`.
//        - Para cada elemento `*i` (lembre-se de desreferenciar), chame a closure `f` passando `*i`.
//        - Se `f(*i)` retornar `true`, adicione `*i` ao vetor `result`.
//        - Ao final, retorne o vetor `result`.
//    - Valor de retorno da função: `Vec<i32>`

// 2. ATUALIZE A FUNÇÃO `main`:
//    - Comente ou remova as chamadas existentes para `odd_filter` e `even_filter`.
//    - Chame a nova função `filter` duas vezes:
//        a. Para filtrar números ÍMPARES:
//           - Passe o vetor `v` e uma closure que implemente a lógica adequada (retorna true se o valor for ímpar).
//           - O resultado esperado é: `[1, 3, 5, 7, 9]`
//           - Imprima o resultado com: `println!("Odd numbers = {:?}", result);`
//        b. Para filtrar números PARES:
//           - Passe o vetor `v` e uma closure que implemente a lógica adequada (retorna true se o valor for par).
//           - O resultado esperado é: `[2, 4, 6, 8, 10]`
//           - Imprima o resultado com: `println!("Even numbers = {:?}", result);`

// 3. REMOVA O CÓDIGO ANTIGO:
//    - Após confirmar que sua nova função `filter` e as chamadas em `main` estão funcionando
//      corretamente e produzindo os outputs esperados, você pode remover as definições
//      originais das funções `odd_filter` e `even_filter`.

fn odd_filter(v: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in v {
        if i % 2 != 0 {
            result.push(*i);
        }
    }
    result
}

fn even_filter(v: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in v {
        if i % 2 == 0 {
            result.push(*i);
        }
    }
    result
}


fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = odd_filter(&v);
    println!("Odd numbers = {:?}", result); // Output: [1, 3, 5]

    let result = even_filter(&v);
    println!("Even numbers = {:?}", result); // Output: [2, 4]
}
