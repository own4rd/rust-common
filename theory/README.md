# Ownership e Gerenciamento de Memória em Rust

## Resumo

O **Ownership** (propriedade) é o sistema exclusivo do Rust para gerenciar memória sem a necessidade de garbage collector ou alocação/liberação manual, como ocorre em linguagens como C ou C++. Em vez disso, Rust aplica regras de compilação que garantem segurança e eficiência no uso da memória.

Esse modelo de propriedade ajuda a manter a segurança da memória sem que o desenvolvedor precise se preocupar constantemente em liberar ou gerenciar memória manualmente.

Embora o conceito seja novo para muitos programadores, com prática ele se torna natural e intuitivo.

### Ownership e Move em Rust

## O que acontece quando movemos uma String?

Em Rust, quando você atribui uma `String` a outra variável, a posse do valor é **movida** para a nova variável. A variável original fica inválida e não pode mais ser usada.

## Exemplo de Código

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Isso causará erro de compilação, pois s1 não é mais válido!
    println!("{s1}, world!");
}


---

## Stack vs Heap

### Stack

- Armazena dados de tamanho fixo e conhecido em tempo de compilação.
- Operações rápidas, baseadas em empilhar e desempilhar em ordem LIFO (Last In, First Out).
- Exemplos de dados armazenados na stack: números primitivos, ponteiros.

### Heap

- Usado para armazenar dados de tamanho dinâmico ou desconhecido durante a compilação.
- Operações mais lentas, pois é necessário buscar espaço livre e manter o controle do uso.
- Exemplos: Strings, vetores (`Vec`), structs dinâmicos.

---

## Como funciona em Rust?

Você guarda ponteiros na stack que apontam para dados armazenados na heap, garantindo eficiência e segurança por meio das regras de ownership.


# Diferença entre String Literal (`&str`) e `String` em Rust

## O que é?

- **String Literal (`&str`)**: texto fixo, imutável e embutido no programa.
- **String (`String`)**: string mutável, armazenada na heap, que pode crescer e mudar em tempo de execução.

## Exemplo de Código

```rust
fn main() {
    // String literal (imutável)
    let literal = "Olá, mundo!";
    println!("Literal: {}", literal);

    // String mutável (alocada na heap)
    let mut string = String::from("Olá, mundo!");
    string.push_str(" Como vai?");
    println!("String: {}", string);
}
