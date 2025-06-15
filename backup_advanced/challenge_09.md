## Desafio 9: Hooks de Runtime - Sistema de Limpeza Automática

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você implementará um pallet que demonstra o uso dos três principais hooks de runtime: `on_initialize`, `on_finalize` e `on_runtime_upgrade`. O pallet gerenciará uma lista de "tarefas temporárias" que são automaticamente limpas após um período determinado, simulando um sistema de garbage collection.

### Conceitos Principais Abordados

1. **Runtime Hooks**: Funções executadas automaticamente em momentos específicos do ciclo de vida dos blocos
2. **`on_initialize`**: Executado no início de cada bloco para preparação
3. **`on_finalize`**: Executado no final de cada bloco para limpeza
4. **`on_runtime_upgrade`**: Executado quando há upgrade do runtime
5. **Weight Management**: Controle de peso nos hooks para não sobrecarregar blocos

### Estruturas a Implementar

#### **`Config` Trait:**
```rust
pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug + Eq + core::hash::Hash;
    type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + 
                     core::ops::Add<Output = Self::BlockNumber> + 
                     core::ops::Sub<Output = Self::BlockNumber> + 
                     core::fmt::Debug;
    type MaxTasksPerBlock: Get<u32>; // Máximo de tarefas a processar por bloco
    type TaskLifetime: Get<Self::BlockNumber>; // Tempo de vida das tarefas
}

pub trait Get<V> {
    fn get() -> V;
}
```

#### **`Task` Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Task<AccountId, BlockNumber> {
    pub id: u32,
    pub creator: AccountId,
    pub created_at: BlockNumber,
    pub data: Vec<u8>,
}
```

#### **`Event<T: Config>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<T: Config> {
    TaskCreated { 
        task_id: u32, 
        creator: T::AccountId, 
        created_at: T::BlockNumber 
    },
    TaskExpired { 
        task_id: u32, 
        expired_at: T::BlockNumber 
    },
    BlockInitialized { 
        block_number: T::BlockNumber, 
        active_tasks: u32 
    },
    BlockFinalized { 
        block_number: T::BlockNumber, 
        tasks_cleaned: u32 
    },
    RuntimeUpgraded { 
        old_version: u32, 
        new_version: u32 
    },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadOrigin,
    TaskDataTooLarge,
    MaxTasksReached,
}
```

#### **`Pallet<T: Config>` Struct:**
```rust
use std::collections::HashMap;

pub struct Pallet<T: Config> {
    tasks: HashMap<u32, Task<T::AccountId, T::BlockNumber>>,
    next_task_id: u32,
    runtime_version: u32,
    emitted_events: Vec<Event<T>>,
    _phantom: core::marker::PhantomData<T>,
}
```

### Métodos Obrigatórios do `Pallet<T: Config>`

#### **Construtor e Utilitários:**
```rust
pub fn new() -> Self {
    Self {
        tasks: HashMap::new(),
        next_task_id: 1,
        runtime_version: 1,
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

fn ensure_signed(origin: Origin<T::AccountId>) -> Result<T::AccountId, Error> {
    match origin {
        Origin::Signed(who) => Ok(who),
    }
}
```

#### **Extrinsic para Criar Tarefas:**
```rust
pub fn create_task(
    &mut self,
    origin: Origin<T::AccountId>,
    data: Vec<u8>,
    current_block: T::BlockNumber,
) -> Result<(), Error> {
    let who = Self::ensure_signed(origin)?;
    
    // Verificar tamanho dos dados
    if data.len() > 1024 {
        return Err(Error::TaskDataTooLarge);
    }
    
    // Verificar limite de tarefas
    if self.tasks.len() >= 1000 {
        return Err(Error::MaxTasksReached);
    }
    
    let task_id = self.next_task_id;
    let task = Task {
        id: task_id,
        creator: who.clone(),
        created_at: current_block,
        data,
    };
    
    self.tasks.insert(task_id, task);
    self.next_task_id += 1;
    
    self.deposit_event(Event::TaskCreated {
        task_id,
        creator: who,
        created_at: current_block,
    });
    
    Ok(())
}
```

#### **Runtime Hooks:**
```rust
// Hook executado no INÍCIO de cada bloco
pub fn on_initialize(&mut self, block_number: T::BlockNumber) -> u64 {
    let active_tasks = self.tasks.len() as u32;
    
    self.deposit_event(Event::BlockInitialized {
        block_number,
        active_tasks,
    });
    
    // Peso base para inicialização
    10_000
}

// Hook executado no FINAL de cada bloco
pub fn on_finalize(&mut self, block_number: T::BlockNumber) -> u64 {
    let max_tasks_to_clean = T::MaxTasksPerBlock::get();
    let task_lifetime = T::TaskLifetime::get();
    let mut tasks_cleaned = 0u32;
    let mut tasks_to_remove = Vec::new();
    
    // Encontrar tarefas expiradas
    for (task_id, task) in &self.tasks {
        if tasks_cleaned >= max_tasks_to_clean {
            break;
        }
        
        let task_age = block_number - task.created_at;
        if task_age >= task_lifetime {
            tasks_to_remove.push(*task_id);
            tasks_cleaned += 1;
        }
    }
    
    // Remover tarefas expiradas
    for task_id in tasks_to_remove {
        self.tasks.remove(&task_id);
        self.deposit_event(Event::TaskExpired {
            task_id,
            expired_at: block_number,
        });
    }
    
    self.deposit_event(Event::BlockFinalized {
        block_number,
        tasks_cleaned,
    });
    
    // Peso baseado no número de tarefas processadas
    10_000 + (tasks_cleaned as u64 * 1_000)
}

// Hook executado durante upgrade do runtime
pub fn on_runtime_upgrade(&mut self) -> u64 {
    let old_version = self.runtime_version;
    let new_version = old_version + 1;
    
    // Simular migração: limpar tarefas muito antigas
    let tasks_before = self.tasks.len();
    self.tasks.retain(|_, task| {
        // Manter apenas tarefas com ID >= 100 (simulação de migração)
        task.id >= 100
    });
    let tasks_after = self.tasks.len();
    
    self.runtime_version = new_version;
    
    self.deposit_event(Event::RuntimeUpgraded {
        old_version,
        new_version,
    });
    
    // Peso baseado no número de tarefas migradas
    50_000 + ((tasks_before - tasks_after) as u64 * 2_000)
}
```

#### **Métodos de Consulta:**
```rust
pub fn get_task(&self, task_id: u32) -> Option<&Task<T::AccountId, T::BlockNumber>> {
    self.tasks.get(&task_id)
}

pub fn active_tasks_count(&self) -> usize {
    self.tasks.len()
}

pub fn runtime_version(&self) -> u32 {
    self.runtime_version
}

pub fn tasks_by_creator(&self, creator: &T::AccountId) -> Vec<&Task<T::AccountId, T::BlockNumber>> {
    self.tasks.values().filter(|task| &task.creator == creator).collect()
}
```

### Origin Enum

```rust
#[derive(Clone, PartialEq, Debug)]
pub enum Origin<AccountId> {
    Signed(AccountId),
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

    struct TestMaxTasksPerBlock;
    impl Get<u32> for TestMaxTasksPerBlock {
        fn get() -> u32 { 5 }
    }

    struct TestTaskLifetime;
    impl Get<TestBlockNumber> for TestTaskLifetime {
        fn get() -> TestBlockNumber { 10 }
    }

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = TestAccountId;
        type BlockNumber = TestBlockNumber;
        type MaxTasksPerBlock = TestMaxTasksPerBlock;
        type TaskLifetime = TestTaskLifetime;
    }

    type TestPallet = Pallet<TestConfig>;
}
```

### Testes Obrigatórios

#### **1. Criação de Tarefas:**
```rust
#[test]
fn test_create_task_success() {
    let mut pallet = TestPallet::new();
    let data = b"test_data".to_vec();
    
    let result = pallet.create_task(Origin::Signed(1), data.clone(), 100);
    assert_eq!(result, Ok(()));
    
    let task = pallet.get_task(1).unwrap();
    assert_eq!(task.creator, 1);
    assert_eq!(task.created_at, 100);
    assert_eq!(task.data, data);
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::TaskCreated {
        task_id: 1,
        creator: 1,
        created_at: 100,
    });
}

#[test]
fn test_create_task_data_too_large() {
    let mut pallet = TestPallet::new();
    let large_data = vec![0u8; 2000]; // Maior que 1024
    
    let result = pallet.create_task(Origin::Signed(1), large_data, 100);
    assert_eq!(result, Err(Error::TaskDataTooLarge));
}
```

#### **2. Hook on_initialize:**
```rust
#[test]
fn test_on_initialize() {
    let mut pallet = TestPallet::new();
    
    // Criar algumas tarefas
    pallet.create_task(Origin::Signed(1), b"task1".to_vec(), 100).unwrap();
    pallet.create_task(Origin::Signed(2), b"task2".to_vec(), 100).unwrap();
    pallet.take_events(); // Limpar eventos
    
    let weight = pallet.on_initialize(105);
    assert_eq!(weight, 10_000);
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::BlockInitialized {
        block_number: 105,
        active_tasks: 2,
    });
}
```

#### **3. Hook on_finalize com Limpeza:**
```rust
#[test]
fn test_on_finalize_cleans_expired_tasks() {
    let mut pallet = TestPallet::new();
    
    // Criar tarefas em blocos diferentes
    pallet.create_task(Origin::Signed(1), b"old_task".to_vec(), 100).unwrap();
    pallet.create_task(Origin::Signed(2), b"new_task".to_vec(), 105).unwrap();
    pallet.take_events(); // Limpar eventos
    
    // Executar finalize no bloco 115 (task1 expira, task2 não)
    let weight = pallet.on_finalize(115);
    
    // Verificar que apenas task1 foi removida
    assert!(pallet.get_task(1).is_none());
    assert!(pallet.get_task(2).is_some());
    assert_eq!(pallet.active_tasks_count(), 1);
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0], Event::TaskExpired {
        task_id: 1,
        expired_at: 115,
    });
    assert_eq!(events[1], Event::BlockFinalized {
        block_number: 115,
        tasks_cleaned: 1,
    });
    
    // Peso deve incluir limpeza
    assert_eq!(weight, 10_000 + 1_000);
}
```

#### **4. Hook on_runtime_upgrade:**
```rust
#[test]
fn test_on_runtime_upgrade() {
    let mut pallet = TestPallet::new();
    
    // Criar tarefas com IDs diferentes
    pallet.next_task_id = 50; // Simular tarefas antigas
    pallet.create_task(Origin::Signed(1), b"old1".to_vec(), 100).unwrap();
    pallet.create_task(Origin::Signed(2), b"old2".to_vec(), 100).unwrap();
    
    pallet.next_task_id = 150; // Simular tarefas novas
    pallet.create_task(Origin::Signed(3), b"new1".to_vec(), 100).unwrap();
    pallet.create_task(Origin::Signed(4), b"new2".to_vec(), 100).unwrap();
    
    assert_eq!(pallet.runtime_version(), 1);
    assert_eq!(pallet.active_tasks_count(), 4);
    pallet.take_events(); // Limpar eventos
    
    let weight = pallet.on_runtime_upgrade();
    
    // Verificar migração
    assert_eq!(pallet.runtime_version(), 2);
    assert_eq!(pallet.active_tasks_count(), 2); // Apenas tarefas com ID >= 100
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::RuntimeUpgraded {
        old_version: 1,
        new_version: 2,
    });
    
    // Peso deve incluir migração
    assert_eq!(weight, 50_000 + (2 * 2_000));
}
```

#### **5. Limite de Tarefas por Bloco:**
```rust
#[test]
fn test_finalize_respects_max_tasks_per_block() {
    let mut pallet = TestPallet::new();
    
    // Criar 10 tarefas expiradas
    for i in 1..=10 {
        pallet.create_task(Origin::Signed(i), b"task".to_vec(), 100).unwrap();
    }
    pallet.take_events(); // Limpar eventos
    
    // Executar finalize - deve limpar apenas 5 (MaxTasksPerBlock)
    let weight = pallet.on_finalize(115);
    
    assert_eq!(pallet.active_tasks_count(), 5); // 10 - 5 = 5
    
    let events = pallet.take_events();
    // 5 TaskExpired + 1 BlockFinalized = 6 eventos
    assert_eq!(events.len(), 6);
    
    // Último evento deve ser BlockFinalized
    if let Event::BlockFinalized { tasks_cleaned, .. } = &events[5] {
        assert_eq!(*tasks_cleaned, 5);
    } else {
        panic!("Expected BlockFinalized event");
    }
}
```

### Critérios de Avaliação

- [ ] Pallet implementado com todos os hooks de runtime
- [ ] `on_initialize` executa no início do bloco
- [ ] `on_finalize` limpa tarefas expiradas respeitando limites
- [ ] `on_runtime_upgrade` realiza migração de dados
- [ ] Controle adequado de peso em todos os hooks
- [ ] Criação de tarefas com validações apropriadas
- [ ] Eventos emitidos corretamente para todos os hooks
- [ ] Todos os testes passam
- [ ] Código bem estruturado e documentado

### Contexto Teórico

**Runtime Hooks** são funções especiais executadas automaticamente pelo runtime do Substrate em momentos específicos do ciclo de vida dos blocos:

**`on_initialize`**:
- Executado no início de cada bloco, antes de qualquer extrinsic
- Usado para preparação, inicialização de estado, verificações
- Deve retornar o peso consumido
- Peso é deduzido do limite total do bloco

**`on_finalize`**:
- Executado no final de cada bloco, após todos os extrinsics
- Usado para limpeza, finalização de estado, garbage collection
- Deve retornar o peso consumido
- Peso é deduzido do limite total do bloco

**`on_runtime_upgrade`**:
- Executado quando há upgrade do runtime
- Usado para migração de dados, limpeza de storage obsoleto
- Crítico para manter compatibilidade entre versões
- Deve ser idempotente (seguro executar múltiplas vezes)

**Weight Management**: É crucial que os hooks não consumam peso excessivo, pois isso pode impedir a inclusão de extrinsics no bloco.

### Próximos Passos

Após completar este desafio:
1. Estude hooks em pallets reais do Substrate
2. Explore estratégias de migração de storage
3. Pratique com weight benchmarking para hooks

### Recursos Adicionais

- [Substrate Runtime Hooks](https://docs.substrate.io/build/runtime-storage/#runtime-hooks)
- [Storage Migrations](https://docs.substrate.io/build/upgrade-the-runtime/)
- [Weight and Fees](https://docs.substrate.io/build/tx-weights-fees/)
