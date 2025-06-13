## Shadowing
A diferença principal entre shadowing e simplesmente usar uma variável mutável (mut) no Rust está na segurança, clareza e controle sobre imutabilidade.


---

## ⚖️ Resumo da Diferença

|                       | `let` Shadowing       | `mut` (mutável)       |
|-----------------------|------------------------|------------------------|
| Cria nova variável?   | ✅ Sim                 | ❌ Não                |
| Pode mudar tipo?      | ✅ Sim                 | ❌ Não                |
| Valor imutável depois? | ✅ Sim (se não usar `mut`) | ❌ Não (continua mutável) |
| Mais seguro?          | ✅ Em geral            | ⚠️ Pode causar bugs se não bem usado |

---