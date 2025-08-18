# Ownership e Gerenciamento de Memória em Rust

## Resumo

O **Ownership** (propriedade) é o sistema exclusivo do Rust para gerenciar memória sem a necessidade de garbage collector ou alocação/liberação manual, como ocorre em linguagens como C ou C++. Em vez disso, Rust aplica regras de compilação que garantem segurança e eficiência no uso da memória.

Esse modelo de propriedade ajuda a manter a segurança da memória sem que o desenvolvedor precise se preocupar constantemente em liberar ou gerenciar memória manualmente.

Embora o conceito seja novo para muitos programadores, com prática ele se torna natural e intuitivo.

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
