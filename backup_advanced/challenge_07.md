## Desafio 7: Simulação de Pesos de Transação com `WeightInfo`

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você implementará um pallet simples de "Armazenamento Chave-Valor" e simulará como os pesos (custos) para suas funções de dispatch (extrinsics) são definidos através de uma `trait WeightInfo`. O objetivo é entender a estrutura e o contrato que um pallet estabelece com o runtime para obter informações de peso.

### Conceitos Principais Abordados

1. **`Weight` Type**: Representação abstrata do custo computacional
2. **`WeightInfo` Trait**: Interface para cálculo de pesos baseado em parâmetros
3. **Integração com Config**: Como o pallet acessa informações de peso
4. **Simulação de Benchmarking**: Estrutura que permite benchmarking real
5. **Parametrização de Peso**: Pesos baseados no tamanho dos dados

### Estruturas a Implementar

#### **`Weight` Type:**
```rust
pub type Weight = u64;
```

#### **`WeightInfo` Trait:**
```rust
pub trait WeightInfo {
    fn store_item(key_len: u32, value_len: u32) -> Weight;
    fn remove_item(key_len: u32) -> Weight;
}
```

#### **`Origin<AccountId>` Enum:**
```rust
#[derive(Clone, PartialEq, Debug)]
pub enum Origin<AccountId> {
    Signed(AccountId),
}
```

#### **`Config` Trait:**
```rust
pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug;
    type WeightInfo: WeightInfo; // Integração crucial com WeightInfo
}
```

#### **`Event<AccountId>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<AccountId> {
    ItemStored { who: AccountId, key: Vec<u8> },
    ItemRemoved { who: AccountId, key: Vec<u8> },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadOrigin,
    ItemNotFound,
}
```

#### **`Pallet<T: Config>` Struct:**
```rust
use std::collections::HashMap;

pub struct Pallet<T: Config> {
    storage: HashMap<Vec<u8>, Vec<u8>>,
    emitted_events: Vec<Event<T::AccountId>>,
    _phantom: core::marker::PhantomData<T>,
}
```

### Métodos Obrigatórios do `Pallet<T: Config>`

#### **Construtor e Utilitários:**
```rust
pub fn new() -> Self {
    Self {
        storage: HashMap::new(),
        emitted_events: Vec::new(),
        _phantom: core::marker::PhantomData,
    }
}

fn deposit_event(&mut self, event: Event<T::AccountId>) {
    self.emitted_events.push(event);
}

pub fn take_events(&mut self) -> Vec<Event<T::AccountId>> {
    std::mem::take(&mut self.emitted_events)
}

fn ensure_signed(origin: Origin<T::AccountId>) -> Result<T::AccountId, Error> {
    match origin {
        Origin::Signed(who) => Ok(who),
    }
}
```

#### **Extrinsics com Integração WeightInfo:**
```rust
pub fn store_item(
    &mut self, 
    origin: Origin<T::AccountId>, 
    key: Vec<u8>, 
    value: Vec<u8>
) -> Result<(), Error> {
    let who = Self::ensure_signed(origin)?;
    
    // Em um pallet real, o peso seria consumido aqui:
    // let weight = T::WeightInfo::store_item(key.len() as u32, value.len() as u32);
    
    self.storage.insert(key.clone(), value);
    self.deposit_event(Event::ItemStored { who, key });
    Ok(())
}

pub fn remove_item(
    &mut self, 
    origin: Origin<T::AccountId>, 
    key: Vec<u8>
) -> Result<(), Error> {
    let who = Self::ensure_signed(origin)?;
    
    // Em um pallet real, o peso seria consumido aqui:
    // let weight = T::WeightInfo::remove_item(key.len() as u32);
    
    if !self.storage.contains_key(&key) {
        return Err(Error::ItemNotFound);
    }
    
    self.storage.remove(&key);
    self.deposit_event(Event::ItemRemoved { who, key });
    Ok(())
}
```

#### **Métodos de Acesso (para testes):**
```rust
pub fn get_item(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
    self.storage.get(key).cloned()
}

pub fn contains_key(&self, key: &Vec<u8>) -> bool {
    self.storage.contains_key(key)
}
```

### Implementação de Teste

#### **Configuração de Teste:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    type TestAccountId = u32;

    struct TestWeightInfo;
    impl WeightInfo for TestWeightInfo {
        fn store_item(key_len: u32, value_len: u32) -> Weight {
            // Peso base + custo por byte
            10_000 + (key_len as u64 * 100) + (value_len as u64 * 200)
        }
        
        fn remove_item(key_len: u32) -> Weight {
            // Peso base + custo por byte da chave
            5_000 + (key_len as u64 * 100)
        }
    }

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = TestAccountId;
        type WeightInfo = TestWeightInfo;
    }

    type TestPallet = Pallet<TestConfig>;
}
```

### Testes Obrigatórios

#### **1. Teste de Configuração:**
```rust
#[test]
fn test_pallet_instantiation() {
    let pallet = TestPallet::new();
    assert_eq!(pallet.take_events().len(), 0);
}
```

#### **2. Teste de Store Item:**
```rust
#[test]
fn test_store_item_success() {
    let mut pallet = TestPallet::new();
    let key = b"test_key".to_vec();
    let value = b"test_value".to_vec();
    
    let result = pallet.store_item(Origin::Signed(1), key.clone(), value.clone());
    assert_eq!(result, Ok(()));
    
    // Verificar se item foi armazenado
    assert_eq!(pallet.get_item(&key), Some(value));
    
    // Verificar evento
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::ItemStored { who: 1, key });
}
```

#### **3. Teste de Remove Item:**
```rust
#[test]
fn test_remove_item_success() {
    let mut pallet = TestPallet::new();
    let key = b"test_key".to_vec();
    let value = b"test_value".to_vec();
    
    // Primeiro armazenar
    pallet.store_item(Origin::Signed(1), key.clone(), value).unwrap();
    pallet.take_events(); // Limpar eventos
    
    // Depois remover
    let result = pallet.remove_item(Origin::Signed(1), key.clone());
    assert_eq!(result, Ok(()));
    
    // Verificar se item foi removido
    assert_eq!(pallet.get_item(&key), None);
    
    // Verificar evento
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::ItemRemoved { who: 1, key });
}
```

#### **4. Teste de Remove Item Inexistente:**
```rust
#[test]
fn test_remove_nonexistent_item() {
    let mut pallet = TestPallet::new();
    let key = b"nonexistent".to_vec();
    
    let result = pallet.remove_item(Origin::Signed(1), key);
    assert_eq!(result, Err(Error::ItemNotFound));
    
    // Verificar que nenhum evento foi emitido
    assert_eq!(pallet.take_events().len(), 0);
}
```

#### **5. Teste de Cálculo de Peso:**
```rust
#[test]
fn test_weight_calculation() {
    // Testar se os pesos são calculados corretamente
    let key_len = 10u32;
    let value_len = 20u32;
    
    let store_weight = TestWeightInfo::store_item(key_len, value_len);
    let expected_store = 10_000 + (10 * 100) + (20 * 200);
    assert_eq!(store_weight, expected_store);
    
    let remove_weight = TestWeightInfo::remove_item(key_len);
    let expected_remove = 5_000 + (10 * 100);
    assert_eq!(remove_weight, expected_remove);
}
```

### Critérios de Avaliação

- [ ] `WeightInfo` trait definida corretamente
- [ ] `Config` trait integra `WeightInfo` adequadamente
- [ ] Pallet implementado com storage HashMap
- [ ] Extrinsics `store_item` e `remove_item` funcionais
- [ ] Verificação de origem implementada
- [ ] Tratamento de erro para item não encontrado
- [ ] Eventos emitidos corretamente
- [ ] Implementação de teste com `TestWeightInfo`
- [ ] Todos os testes passam
- [ ] Código bem estruturado e documentado

### Contexto Teórico

**Weight (Peso)** no Substrate é uma medida abstrata do tempo e recursos que uma operação consome. É fundamental para:
- Garantir que blocos não sejam sobrecarregados
- Calcular taxas de transação
- Prevenir ataques de DoS

**`WeightInfo` Trait** é tipicamente gerada por ferramentas de benchmarking que:
- Medem o tempo real de execução de extrinsics
- Consideram diferentes parâmetros de entrada
- Geram funções que calculam peso baseado nesses parâmetros

**Integração com Runtime**: O runtime fornece uma implementação concreta da `WeightInfo` trait, permitindo que o pallet acesse informações de peso sem conhecer os detalhes de implementação.

**Macro `#[pallet::weight]`**: Em pallets reais, você usaria:
```rust
#[pallet::weight(T::WeightInfo::store_item(key.len() as u32, value.len() as u32))]
pub fn store_item(origin, key: Vec<u8>, value: Vec<u8>) -> DispatchResult {
    // implementação
}
```

### Próximos Passos

Após completar este desafio:
1. Estude ferramentas de benchmarking do Substrate
2. Explore como pesos são usados para calcular taxas
3. Pratique com benchmarking real usando `frame-benchmarking`

### Recursos Adicionais

- [Substrate Weights Documentation](https://docs.substrate.io/build/tx-weights-fees/)
- [Benchmarking Guide](https://docs.substrate.io/test/benchmark/)
- [Weight Examples](https://github.com/paritytech/substrate/tree/master/frame/examples)
