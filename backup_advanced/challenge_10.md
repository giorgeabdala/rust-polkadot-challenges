## Desafio 10: Inherents - Timestamp e Dados Externos

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você implementará um pallet que simula o sistema de Inherents do Substrate. Inherents são dados que devem ser incluídos em cada bloco (como timestamp, número do bloco, etc.) e são fornecidos pelos block authors. Você criará um sistema que valida e processa inherents de timestamp e dados de consenso.

### Conceitos Principais Abordados

1. **Inherents**: Dados obrigatórios que devem estar presentes em cada bloco
2. **InherentData**: Estrutura que carrega dados externos para o bloco
3. **Validação de Inherents**: Verificação se os dados fornecidos são válidos
4. **Timestamp Inherent**: Timestamp obrigatório em cada bloco
5. **Consensus Data**: Dados de consenso que acompanham cada bloco

### Estruturas a Implementar

#### **`InherentIdentifier`:**
```rust
pub type InherentIdentifier = [u8; 8];

// Identificadores para diferentes tipos de inherents
pub const TIMESTAMP_INHERENT_IDENTIFIER: InherentIdentifier = *b"timstap0";
pub const CONSENSUS_INHERENT_IDENTIFIER: InherentIdentifier = *b"consensu";
```

#### **`InherentData` Struct:**
```rust
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct InherentData {
    data: HashMap<InherentIdentifier, Vec<u8>>,
}

impl InherentData {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    pub fn put_data(&mut self, identifier: InherentIdentifier, data: Vec<u8>) {
        self.data.insert(identifier, data);
    }
    
    pub fn get_data(&self, identifier: &InherentIdentifier) -> Option<&Vec<u8>> {
        self.data.get(identifier)
    }
    
    pub fn has_data(&self, identifier: &InherentIdentifier) -> bool {
        self.data.contains_key(identifier)
    }
}
```

#### **`Config` Trait:**
```rust
pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug;
    type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug;
    type Timestamp: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug;
    type MinTimestampDelta: Get<Self::Timestamp>; // Mínimo de tempo entre blocos
}

pub trait Get<V> {
    fn get() -> V;
}
```

#### **`Event<T: Config>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<T: Config> {
    TimestampSet { 
        timestamp: T::Timestamp,
        block_number: T::BlockNumber 
    },
    ConsensusDataProcessed { 
        data_size: u32,
        block_number: T::BlockNumber 
    },
    InherentValidationFailed { 
        identifier: InherentIdentifier,
        reason: String 
    },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    MissingTimestampInherent,
    InvalidTimestamp,
    TimestampTooEarly,
    TimestampTooFarInFuture,
    MissingConsensusData,
    InvalidConsensusData,
    InherentDataCorrupted,
}
```

#### **`Pallet<T: Config>` Struct:**
```rust
pub struct Pallet<T: Config> {
    current_timestamp: Option<T::Timestamp>,
    last_timestamp: Option<T::Timestamp>,
    consensus_data: Option<Vec<u8>>,
    emitted_events: Vec<Event<T>>,
    _phantom: core::marker::PhantomData<T>,
}
```

### Métodos Obrigatórios do `Pallet<T: Config>`

#### **Construtor e Utilitários:**
```rust
pub fn new() -> Self {
    Self {
        current_timestamp: None,
        last_timestamp: None,
        consensus_data: None,
        emitted_events: Vec::new(),
        _phantom: core::marker::PhantomData,
    }
}

fn deposit_event(&mut self, event: Event<T>) {
    self.emitted_events.push(event);
}

pub fn take_events(&mut self) -> Vec<Event<T>> {
    std::mem::take(&mut self.emitted_events)
}
```

#### **Processamento de Inherents:**
```rust
pub fn create_inherents(
    &self,
    timestamp: T::Timestamp,
    consensus_data: Vec<u8>,
) -> InherentData {
    let mut inherent_data = InherentData::new();
    
    // Serializar timestamp (simulação simples)
    let timestamp_bytes = format!("{:?}", timestamp).into_bytes();
    inherent_data.put_data(TIMESTAMP_INHERENT_IDENTIFIER, timestamp_bytes);
    
    // Adicionar dados de consenso
    inherent_data.put_data(CONSENSUS_INHERENT_IDENTIFIER, consensus_data);
    
    inherent_data
}

pub fn check_inherents(
    &self,
    inherent_data: &InherentData,
    current_time: T::Timestamp,
) -> Result<(), Error> {
    // Verificar se timestamp inherent está presente
    if !inherent_data.has_data(&TIMESTAMP_INHERENT_IDENTIFIER) {
        return Err(Error::MissingTimestampInherent);
    }
    
    // Verificar se consensus data está presente
    if !inherent_data.has_data(&CONSENSUS_INHERENT_IDENTIFIER) {
        return Err(Error::MissingConsensusData);
    }
    
    // Validar timestamp
    let timestamp_data = inherent_data.get_data(&TIMESTAMP_INHERENT_IDENTIFIER)
        .ok_or(Error::MissingTimestampInherent)?;
    
    // Deserializar timestamp (simulação simples)
    let timestamp_str = String::from_utf8(timestamp_data.clone())
        .map_err(|_| Error::InherentDataCorrupted)?;
    
    // Validar se timestamp não é muito antigo
    if let Some(last_ts) = self.last_timestamp {
        let min_delta = T::MinTimestampDelta::get();
        // Simulação de validação de tempo mínimo
        // Em implementação real, seria: timestamp >= last_ts + min_delta
    }
    
    // Validar consensus data
    let consensus_data = inherent_data.get_data(&CONSENSUS_INHERENT_IDENTIFIER)
        .ok_or(Error::MissingConsensusData)?;
    
    if consensus_data.is_empty() {
        return Err(Error::InvalidConsensusData);
    }
    
    Ok(())
}

pub fn process_inherents(
    &mut self,
    inherent_data: InherentData,
    block_number: T::BlockNumber,
) -> Result<(), Error> {
    // Primeiro validar
    let current_time = self.current_timestamp.unwrap_or_default();
    self.check_inherents(&inherent_data, current_time)?;
    
    // Processar timestamp
    if let Some(timestamp_data) = inherent_data.get_data(&TIMESTAMP_INHERENT_IDENTIFIER) {
        let timestamp_str = String::from_utf8(timestamp_data.clone())
            .map_err(|_| Error::InherentDataCorrupted)?;
        
        // Atualizar timestamps
        self.last_timestamp = self.current_timestamp;
        // Em implementação real, deserializaria o timestamp corretamente
        // Por simplicidade, usamos um valor padrão
        self.current_timestamp = Some(T::Timestamp::default());
        
        self.deposit_event(Event::TimestampSet {
            timestamp: self.current_timestamp.unwrap(),
            block_number,
        });
    }
    
    // Processar consensus data
    if let Some(consensus_data) = inherent_data.get_data(&CONSENSUS_INHERENT_IDENTIFIER) {
        self.consensus_data = Some(consensus_data.clone());
        
        self.deposit_event(Event::ConsensusDataProcessed {
            data_size: consensus_data.len() as u32,
            block_number,
        });
    }
    
    Ok(())
}
```

#### **Métodos de Consulta:**
```rust
pub fn current_timestamp(&self) -> Option<T::Timestamp> {
    self.current_timestamp
}

pub fn last_timestamp(&self) -> Option<T::Timestamp> {
    self.last_timestamp
}

pub fn consensus_data(&self) -> Option<&Vec<u8>> {
    self.consensus_data.as_ref()
}

pub fn consensus_data_size(&self) -> usize {
    self.consensus_data.as_ref().map_or(0, |data| data.len())
}
```

### Implementação de Teste

#### **Configuração de Teste:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    type TestAccountId = u32;
    type TestBlockNumber = u64;
    type TestTimestamp = u64;

    struct TestMinTimestampDelta;
    impl Get<TestTimestamp> for TestMinTimestampDelta {
        fn get() -> TestTimestamp { 1000 } // 1 segundo mínimo
    }

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = TestAccountId;
        type BlockNumber = TestBlockNumber;
        type Timestamp = TestTimestamp;
        type MinTimestampDelta = TestMinTimestampDelta;
    }

    type TestPallet = Pallet<TestConfig>;
}
```

### Testes Obrigatórios

#### **1. Criação de Inherent Data:**
```rust
#[test]
fn test_create_inherents() {
    let pallet = TestPallet::new();
    let timestamp = 1000u64;
    let consensus_data = b"consensus_info".to_vec();
    
    let inherent_data = pallet.create_inherents(timestamp, consensus_data.clone());
    
    assert!(inherent_data.has_data(&TIMESTAMP_INHERENT_IDENTIFIER));
    assert!(inherent_data.has_data(&CONSENSUS_INHERENT_IDENTIFIER));
    
    let stored_consensus = inherent_data.get_data(&CONSENSUS_INHERENT_IDENTIFIER).unwrap();
    assert_eq!(*stored_consensus, consensus_data);
}
```

#### **2. Validação de Inherents Válidos:**
```rust
#[test]
fn test_check_inherents_valid() {
    let pallet = TestPallet::new();
    let mut inherent_data = InherentData::new();
    
    // Adicionar dados válidos
    inherent_data.put_data(TIMESTAMP_INHERENT_IDENTIFIER, b"1000".to_vec());
    inherent_data.put_data(CONSENSUS_INHERENT_IDENTIFIER, b"valid_data".to_vec());
    
    let result = pallet.check_inherents(&inherent_data, 1000);
    assert_eq!(result, Ok(()));
}
```

#### **3. Validação de Inherents Inválidos:**
```rust
#[test]
fn test_check_inherents_missing_timestamp() {
    let pallet = TestPallet::new();
    let mut inherent_data = InherentData::new();
    
    // Apenas consensus data, sem timestamp
    inherent_data.put_data(CONSENSUS_INHERENT_IDENTIFIER, b"data".to_vec());
    
    let result = pallet.check_inherents(&inherent_data, 1000);
    assert_eq!(result, Err(Error::MissingTimestampInherent));
}

#[test]
fn test_check_inherents_missing_consensus() {
    let pallet = TestPallet::new();
    let mut inherent_data = InherentData::new();
    
    // Apenas timestamp, sem consensus data
    inherent_data.put_data(TIMESTAMP_INHERENT_IDENTIFIER, b"1000".to_vec());
    
    let result = pallet.check_inherents(&inherent_data, 1000);
    assert_eq!(result, Err(Error::MissingConsensusData));
}

#[test]
fn test_check_inherents_empty_consensus() {
    let pallet = TestPallet::new();
    let mut inherent_data = InherentData::new();
    
    inherent_data.put_data(TIMESTAMP_INHERENT_IDENTIFIER, b"1000".to_vec());
    inherent_data.put_data(CONSENSUS_INHERENT_IDENTIFIER, Vec::new()); // Vazio
    
    let result = pallet.check_inherents(&inherent_data, 1000);
    assert_eq!(result, Err(Error::InvalidConsensusData));
}
```

#### **4. Processamento de Inherents:**
```rust
#[test]
fn test_process_inherents_success() {
    let mut pallet = TestPallet::new();
    let mut inherent_data = InherentData::new();
    
    inherent_data.put_data(TIMESTAMP_INHERENT_IDENTIFIER, b"1000".to_vec());
    inherent_data.put_data(CONSENSUS_INHERENT_IDENTIFIER, b"consensus_data".to_vec());
    
    let result = pallet.process_inherents(inherent_data, 100);
    assert_eq!(result, Ok(()));
    
    // Verificar se timestamp foi atualizado
    assert!(pallet.current_timestamp().is_some());
    
    // Verificar se consensus data foi armazenado
    assert_eq!(pallet.consensus_data_size(), 14); // "consensus_data".len()
    
    // Verificar eventos
    let events = pallet.take_events();
    assert_eq!(events.len(), 2);
    
    assert!(matches!(events[0], Event::TimestampSet { .. }));
    assert!(matches!(events[1], Event::ConsensusDataProcessed { .. }));
}
```

#### **5. Sequência de Blocos:**
```rust
#[test]
fn test_multiple_blocks_sequence() {
    let mut pallet = TestPallet::new();
    
    // Primeiro bloco
    let mut inherent_data1 = InherentData::new();
    inherent_data1.put_data(TIMESTAMP_INHERENT_IDENTIFIER, b"1000".to_vec());
    inherent_data1.put_data(CONSENSUS_INHERENT_IDENTIFIER, b"block1_data".to_vec());
    
    pallet.process_inherents(inherent_data1, 1).unwrap();
    let first_timestamp = pallet.current_timestamp();
    pallet.take_events(); // Limpar eventos
    
    // Segundo bloco
    let mut inherent_data2 = InherentData::new();
    inherent_data2.put_data(TIMESTAMP_INHERENT_IDENTIFIER, b"2000".to_vec());
    inherent_data2.put_data(CONSENSUS_INHERENT_IDENTIFIER, b"block2_data".to_vec());
    
    pallet.process_inherents(inherent_data2, 2).unwrap();
    
    // Verificar que last_timestamp foi atualizado
    assert_eq!(pallet.last_timestamp(), first_timestamp);
    assert!(pallet.current_timestamp() != first_timestamp);
    
    // Verificar novos dados de consenso
    assert_eq!(pallet.consensus_data_size(), 11); // "block2_data".len()
}
```

#### **6. InherentData Utility Methods:**
```rust
#[test]
fn test_inherent_data_operations() {
    let mut inherent_data = InherentData::new();
    
    // Testar put e get
    let test_data = b"test_data".to_vec();
    inherent_data.put_data(TIMESTAMP_INHERENT_IDENTIFIER, test_data.clone());
    
    assert!(inherent_data.has_data(&TIMESTAMP_INHERENT_IDENTIFIER));
    assert!(!inherent_data.has_data(&CONSENSUS_INHERENT_IDENTIFIER));
    
    let retrieved = inherent_data.get_data(&TIMESTAMP_INHERENT_IDENTIFIER);
    assert_eq!(retrieved, Some(&test_data));
    
    let missing = inherent_data.get_data(&CONSENSUS_INHERENT_IDENTIFIER);
    assert_eq!(missing, None);
}
```

### Critérios de Avaliação

- [ ] `InherentData` struct implementada com métodos de manipulação
- [ ] Pallet processa inherents de timestamp e consensus data
- [ ] Validação adequada de inherents obrigatórios
- [ ] Método `create_inherents` gera dados válidos
- [ ] Método `check_inherents` valida corretamente
- [ ] Método `process_inherents` atualiza estado do pallet
- [ ] Tratamento de erros para dados inválidos ou ausentes
- [ ] Eventos emitidos corretamente
- [ ] Todos os testes passam
- [ ] Código bem estruturado e documentado

### Contexto Teórico

**Inherents** no Substrate são dados que devem estar presentes em cada bloco e são fornecidos pelos block authors (validadores/mineradores). Diferentemente dos extrinsics, inherents não são assinados e não vêm de usuários externos.

**Características dos Inherents**:
- **Obrigatórios**: Devem estar presentes em cada bloco
- **Não assinados**: Não requerem assinatura criptográfica
- **Fornecidos pelo Author**: O block author é responsável por incluí-los
- **Validados**: O runtime valida se os inherents são corretos

**Tipos Comuns de Inherents**:
- **Timestamp**: Tempo atual do bloco
- **Block Number**: Número do bloco atual
- **Consensus Data**: Dados específicos do algoritmo de consenso
- **Uncle Blocks**: Blocos órfãos (em alguns consensos)

**Fluxo de Processamento**:
1. Block author coleta dados externos (tempo, consensus info)
2. Cria `InherentData` com os dados necessários
3. Inclui inherents no bloco
4. Runtime valida inherents durante execução do bloco
5. Se válidos, processa e atualiza estado

**Validação**: É crucial validar inherents para prevenir ataques onde block authors fornecem dados incorretos (ex: timestamps muito antigos ou futuros).

### Próximos Passos

Após completar este desafio:
1. Estude inherents em pallets reais (Timestamp, Aura, BABE)
2. Explore como diferentes algoritmos de consenso usam inherents
3. Pratique com validação de timestamp em cenários reais

### Recursos Adicionais

- [Substrate Inherents](https://docs.substrate.io/build/custom-pallets/#inherents)
- [Timestamp Pallet](https://github.com/paritytech/substrate/tree/master/frame/timestamp)
- [Inherent Data Providers](https://docs.substrate.io/build/custom-pallets/#inherent-data-providers)
