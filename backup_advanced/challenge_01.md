## Desafio 1: Pallet Básico de Contador (Revisão de Fundamentos)

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você implementará um "pallet" de contador em Rust puro. O objetivo é simular a estrutura e a lógica interna de um pallet do Substrate, focando nos seguintes conceitos e estruturas de dados do Rust e do FRAME (simulados):

### Conceitos Principais Abordados

1. **`Structs` e `Enums`:**
   - Struct `Pallet<T: Config>` para encapsular a lógica do contador
   - Trait `Config` para a configuração do pallet
   - `enum Event<AccountId>` para os eventos que o pallet pode emitir
   - `enum Error` para os possíveis erros das operações
   - `enum Origin<AccountId>` para simular as origens das chamadas (Root, Signed)

2. **`Traits` e `Generics`:**
   - A `Config` trait será genérica e parametrizará o `Pallet`
   - Trait auxiliar `Get<V>` para obter valores de configuração
   - Tipos associados: `AccountId` e `MaxValue: Get<u32>`

3. **Simulação de Storage:**
   - Campo para armazenar o valor atual do contador (simulando `StorageValue`)
   - Lista dos eventos emitidos para testes

4. **Funções de Dispatch (Extrinsics Simulados):**
   - `set_value`: Apenas `Root` pode definir valores
   - `increment`: Apenas `Signed` pode incrementar
   - `decrement`: Apenas `Signed` pode decrementar  
   - `reset`: Apenas `Root` pode resetar

5. **Tratamento de Erros:**
   - `Option<T>` e `Result<T, E>` para valores ausentes e erros
   - `Pattern Matching` para tratar `Result`s, `Option`s e `Origin`s

### Estruturas a Implementar

#### **Trait Auxiliar `Get<V>`:**
```rust
pub trait Get<V> {
    fn get() -> V;
}
```

#### **`Origin<AccountId>` Enum:**
```rust
#[derive(Clone, PartialEq, Debug)]
pub enum Origin<AccountId> {
    Root,
    Signed(AccountId),
}
```

#### **`Config` Trait:**
```rust
pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug;
    type MaxValue: Get<u32>; // Valor máximo para o contador
}
```

#### **`Event<AccountId>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<AccountId> {
    ValueSet { new_value: u32, who: Option<AccountId> },
    Incremented { new_value: u32, who: AccountId, amount: u32 },
    Decremented { new_value: u32, who: AccountId, amount: u32 },
    Reset { who: Option<AccountId> },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadOrigin,
    SetValueTooHigh,
    Overflow,
    Underflow,
}
```

#### **`Pallet<T: Config>` Struct:**
```rust
pub struct Pallet<T: Config> {
    value: u32,
    emitted_events: Vec<Event<T::AccountId>>,
    _phantom: core::marker::PhantomData<T>,
}
```

### Métodos Obrigatórios do `Pallet<T: Config>`

- `pub fn new() -> Self` - Inicializa contador com `0`
- `pub fn set_value(&mut self, origin: Origin<T::AccountId>, new_value: u32) -> Result<(), Error>`
- `pub fn increment(&mut self, origin: Origin<T::AccountId>, amount: u32) -> Result<(), Error>`
- `pub fn decrement(&mut self, origin: Origin<T::AccountId>, amount: u32) -> Result<(), Error>`
- `pub fn reset(&mut self, origin: Origin<T::AccountId>) -> Result<(), Error>`
- `fn deposit_event(&mut self, event: Event<T::AccountId>)`
- `pub fn take_events(&mut self) -> Vec<Event<T::AccountId>>`

### Funções Auxiliares Recomendadas

```rust
fn ensure_root(origin: Origin<T::AccountId>) -> Result<(), Error> {
    match origin {
        Origin::Root => Ok(()),
        _ => Err(Error::BadOrigin),
    }
}

fn ensure_signed(origin: Origin<T::AccountId>) -> Result<T::AccountId, Error> {
    match origin {
        Origin::Signed(who) => Ok(who),
        _ => Err(Error::BadOrigin),
    }
}
```

### Testes Obrigatórios

Implemente os seguintes cenários de teste:

#### **Configuração de Teste:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    type TestAccountId = u32;
    
    struct TestMaxValue;
    impl Get<u32> for TestMaxValue {
        fn get() -> u32 { 100 }
    }
    
    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = TestAccountId;
        type MaxValue = TestMaxValue;
    }

    // Seus testes aqui...
}
```

#### **Cenários de Teste:**

1. **Inicialização:**
   - Verificar se o contador é inicializado com `0`

2. **`set_value`:**
   - ✅ Sucesso: `Root` define um valor válido
   - ❌ Erro: `Signed` tenta definir valor (`Error::BadOrigin`)
   - ❌ Erro: `Root` tenta definir valor acima de `MaxValue` (`Error::SetValueTooHigh`)

3. **`increment`:**
   - ✅ Sucesso: `Signed` incrementa valor
   - ❌ Erro: `Root` tenta incrementar (`Error::BadOrigin`)
   - ❌ Erro: Incremento causa `Overflow`
   - ❌ Erro: Incremento excede `MaxValue`

4. **`decrement`:**
   - ✅ Sucesso: `Signed` decrementa valor
   - ❌ Erro: `Root` tenta decrementar (`Error::BadOrigin`)
   - ❌ Erro: Decremento causa `Underflow`

5. **`reset`:**
   - ✅ Sucesso: `Root` reseta o valor
   - ❌ Erro: `Signed` tenta resetar (`Error::BadOrigin`)

6. **Sequência de Operações:**
   - Testar múltiplas operações em sequência

7. **Coleta de Eventos:**
   - Verificar se `take_events()` funciona corretamente

### Critérios de Avaliação

- [ ] Todas as structs e enums implementadas corretamente
- [ ] Funções de dispatch com verificação de origem adequada
- [ ] Tratamento correto de overflow/underflow
- [ ] Verificação de `MaxValue`
- [ ] Todos os eventos emitidos corretamente
- [ ] Todos os testes passando
- [ ] Código bem estruturado e documentado

### Contexto Teórico

Este desafio simula a estrutura essencial de um pallet FRAME em Rust puro. Em um ambiente Substrate real, você usaria macros que automatizam muito do boilerplate:

**Pallet Structure**: Normalmente um pallet é um módulo Rust anotado com `#[frame::pallet]`. A struct `Pallet<T>(_)` serve como um tipo para o qual as funções de dispatch são associadas.

**Configuration Trait**: A trait `Config` define os tipos e constantes que o pallet precisa do runtime. Em pallets reais, incluiria `type RuntimeEvent: From<Event<Self>>` e constantes como `#[pallet::constant] type CounterMaxValue: Get<u32>`.

**Events**: Eventos sinalizam que algo significativo aconteceu. A macro `#[pallet::generate_deposit]` cria uma função helper `deposit_event` automaticamente.

**Storage**: O FRAME fornece tipos como `StorageValue<_, u32>` e `StorageMap<_, Blake2_128Concat, T::AccountId, u32>`. Estamos simulando com campos simples na struct.

**Dispatchable Calls**: Funções anotadas com `#[pallet::call]` são os extrinsics que usuários podem chamar. Retornam `DispatchResult` (que é um `Result<(), DispatchError>`).

### Próximos Passos

Após completar este desafio:
1. Estude a documentação oficial sobre FRAME pallets
2. Explore o código de pallets existentes no repositório Substrate
3. Pratique com as macros reais do FRAME

### Recursos Adicionais

- [FRAME Pallet Documentation](https://docs.substrate.io/reference/frame-pallets/)
- [Substrate Pallet Template](https://github.com/substrate-developer-hub/substrate-node-template)
- [FRAME Macros Reference](https://docs.substrate.io/reference/frame-macros/)
