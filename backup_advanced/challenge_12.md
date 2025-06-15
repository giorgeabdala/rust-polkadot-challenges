## Desafio 12: Substrate Node Template e Configuração de Runtime

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio final, você implementará uma simulação completa de um Substrate Runtime, integrando múltiplos pallets em um sistema coeso. O foco é entender como pallets são configurados, conectados e como o runtime é construído para formar uma blockchain funcional.

### Conceitos Principais Abordados

1. **Runtime Construction**: Como construir um runtime completo
2. **Pallet Integration**: Integração de múltiplos pallets
3. **Runtime Configuration**: Configuração de parâmetros do runtime
4. **Genesis Configuration**: Estado inicial da blockchain
5. **Runtime APIs**: Interfaces para consulta externa

### Estruturas a Implementar

#### **Tipos Básicos do Runtime:**
```rust
// Tipos fundamentais do runtime
pub type AccountId = String; // Simplificado
pub type BlockNumber = u64;
pub type Hash = [u8; 32];
pub type Balance = u128;
pub type Nonce = u32;

// Header do bloco
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub number: BlockNumber,
    pub parent_hash: Hash,
    pub state_root: Hash,
    pub extrinsics_root: Hash,
}

// Bloco completo
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub header: Header,
    pub extrinsics: Vec<Vec<u8>>, // Extrinsics serializados
}
```

#### **System Pallet (Simplificado):**
```rust
pub mod system {
    use super::*;
    use std::collections::HashMap;

    pub trait Config {
        type AccountId: Clone + PartialEq + core::fmt::Debug;
        type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug;
        type Hash: Clone + PartialEq + core::fmt::Debug;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        NewAccount { account: T::AccountId },
        BlockFinalized { number: T::BlockNumber },
    }

    pub struct Pallet<T: Config> {
        account_nonces: HashMap<T::AccountId, u32>,
        current_block_number: T::BlockNumber,
        block_hash: HashMap<T::BlockNumber, T::Hash>,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                account_nonces: HashMap::new(),
                current_block_number: T::BlockNumber::default(),
                block_hash: HashMap::new(),
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn inc_account_nonce(&mut self, account: &T::AccountId) {
            let nonce = self.account_nonces.entry(account.clone()).or_insert(0);
            *nonce += 1;
            
            if *nonce == 1 {
                self.events.push(Event::NewAccount { account: account.clone() });
            }
        }

        pub fn account_nonce(&self, account: &T::AccountId) -> u32 {
            self.account_nonces.get(account).copied().unwrap_or(0)
        }

        pub fn finalize_block(&mut self, number: T::BlockNumber, hash: T::Hash) {
            self.current_block_number = number;
            self.block_hash.insert(number, hash);
            self.events.push(Event::BlockFinalized { number });
        }

        pub fn block_number(&self) -> T::BlockNumber {
            self.current_block_number
        }

        pub fn block_hash(&self, number: T::BlockNumber) -> Option<T::Hash> {
            self.block_hash.get(&number).copied()
        }

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}
```

#### **Balances Pallet (Simplificado):**
```rust
pub mod balances {
    use super::*;
    use std::collections::HashMap;

    pub trait Config: system::Config {
        type Balance: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug +
                     core::ops::Add<Output = Self::Balance> + 
                     core::ops::Sub<Output = Self::Balance>;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        Transfer { 
            from: T::AccountId, 
            to: T::AccountId, 
            amount: T::Balance 
        },
        BalanceSet { 
            account: T::AccountId, 
            balance: T::Balance 
        },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        InsufficientBalance,
        AccountNotFound,
    }

    pub struct Pallet<T: Config> {
        balances: HashMap<T::AccountId, T::Balance>,
        total_issuance: T::Balance,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                balances: HashMap::new(),
                total_issuance: T::Balance::default(),
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn set_balance(&mut self, account: T::AccountId, balance: T::Balance) {
            let old_balance = self.balances.get(&account).copied().unwrap_or_default();
            self.balances.insert(account.clone(), balance);
            
            // Ajustar total issuance
            if balance > old_balance {
                self.total_issuance = self.total_issuance + (balance - old_balance);
            } else if old_balance > balance {
                self.total_issuance = self.total_issuance - (old_balance - balance);
            }

            self.events.push(Event::BalanceSet { account, balance });
        }

        pub fn transfer(
            &mut self,
            from: T::AccountId,
            to: T::AccountId,
            amount: T::Balance,
        ) -> Result<(), Error> {
            let from_balance = self.balances.get(&from).copied().unwrap_or_default();
            
            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            let to_balance = self.balances.get(&to).copied().unwrap_or_default();

            self.balances.insert(from.clone(), from_balance - amount);
            self.balances.insert(to.clone(), to_balance + amount);

            self.events.push(Event::Transfer { from, to, amount });
            Ok(())
        }

        pub fn balance(&self, account: &T::AccountId) -> T::Balance {
            self.balances.get(account).copied().unwrap_or_default()
        }

        pub fn total_issuance(&self) -> T::Balance {
            self.total_issuance
        }

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}
```

#### **Custom Pallet (Exemplo):**
```rust
pub mod custom_pallet {
    use super::*;
    use std::collections::HashMap;

    pub trait Config: system::Config + balances::Config {
        type MaxDataLength: Get<u32>;
    }

    pub trait Get<V> {
        fn get() -> V;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct CustomData {
        pub owner: String, // AccountId simplificado
        pub data: Vec<u8>,
        pub created_at: u64, // BlockNumber simplificado
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        DataStored { 
            id: u32, 
            owner: T::AccountId 
        },
        DataUpdated { 
            id: u32, 
            owner: T::AccountId 
        },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        DataTooLarge,
        DataNotFound,
        NotOwner,
    }

    pub struct Pallet<T: Config> {
        data_storage: HashMap<u32, CustomData>,
        next_id: u32,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                data_storage: HashMap::new(),
                next_id: 1,
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn store_data(
            &mut self,
            owner: T::AccountId,
            data: Vec<u8>,
            block_number: u64,
        ) -> Result<u32, Error> {
            if data.len() > T::MaxDataLength::get() as usize {
                return Err(Error::DataTooLarge);
            }

            let id = self.next_id;
            let custom_data = CustomData {
                owner: format!("{:?}", owner), // Conversão simplificada
                data,
                created_at: block_number,
            };

            self.data_storage.insert(id, custom_data);
            self.next_id += 1;

            self.events.push(Event::DataStored { id, owner });
            Ok(id)
        }

        pub fn get_data(&self, id: u32) -> Option<&CustomData> {
            self.data_storage.get(&id)
        }

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}
```

#### **Runtime Principal:**
```rust
// Configuração do Runtime
pub struct Runtime;

// Implementar Config para cada pallet
impl system::Config for Runtime {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
}

impl balances::Config for Runtime {
    type Balance = Balance;
}

struct MaxDataLength;
impl Get<u32> for MaxDataLength {
    fn get() -> u32 { 1024 }
}

impl custom_pallet::Config for Runtime {
    type MaxDataLength = MaxDataLength;
}

// Runtime struct que contém todos os pallets
pub struct RuntimeInstance {
    pub system: system::Pallet<Runtime>,
    pub balances: balances::Pallet<Runtime>,
    pub custom_pallet: custom_pallet::Pallet<Runtime>,
}

impl RuntimeInstance {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            custom_pallet: custom_pallet::Pallet::new(),
        }
    }

    // Genesis configuration
    pub fn initialize_genesis(&mut self, genesis_config: GenesisConfig) {
        // Configurar balances iniciais
        for (account, balance) in genesis_config.balances {
            self.balances.set_balance(account, balance);
        }

        // Finalizar bloco genesis
        let genesis_hash = [0u8; 32]; // Hash simplificado
        self.system.finalize_block(0, genesis_hash);
    }

    // Executar um bloco
    pub fn execute_block(&mut self, block: Block) -> Result<(), String> {
        let block_number = block.header.number;
        
        // Processar extrinsics (simulado)
        for extrinsic in block.extrinsics {
            self.process_extrinsic(extrinsic, block_number)?;
        }

        // Finalizar bloco
        self.system.finalize_block(block_number, block.header.state_root);
        
        Ok(())
    }

    fn process_extrinsic(&mut self, extrinsic: Vec<u8>, block_number: BlockNumber) -> Result<(), String> {
        // Simulação simples de processamento de extrinsic
        // Em implementação real, seria deserializado e roteado para o pallet correto
        
        if extrinsic.starts_with(b"transfer:") {
            // Simular transfer
            // Format: "transfer:from:to:amount"
            let parts: Vec<&str> = std::str::from_utf8(&extrinsic)
                .map_err(|_| "Invalid extrinsic format")?
                .split(':')
                .collect();
            
            if parts.len() == 4 {
                let from = parts[1].to_string();
                let to = parts[2].to_string();
                let amount: Balance = parts[3].parse().map_err(|_| "Invalid amount")?;
                
                self.system.inc_account_nonce(&from);
                self.balances.transfer(from, to, amount)
                    .map_err(|_| "Transfer failed")?;
            }
        } else if extrinsic.starts_with(b"store_data:") {
            // Simular store_data
            // Format: "store_data:owner:data"
            let parts: Vec<&str> = std::str::from_utf8(&extrinsic)
                .map_err(|_| "Invalid extrinsic format")?
                .split(':')
                .collect();
            
            if parts.len() == 3 {
                let owner = parts[1].to_string();
                let data = parts[2].as_bytes().to_vec();
                
                self.system.inc_account_nonce(&owner);
                self.custom_pallet.store_data(owner, data, block_number)
                    .map_err(|_| "Store data failed")?;
            }
        }

        Ok(())
    }

    // Coletar todos os eventos
    pub fn collect_events(&mut self) -> RuntimeEvents {
        RuntimeEvents {
            system: self.system.take_events(),
            balances: self.balances.take_events(),
            custom_pallet: self.custom_pallet.take_events(),
        }
    }

    // Runtime APIs (consultas)
    pub fn account_balance(&self, account: &AccountId) -> Balance {
        self.balances.balance(account)
    }

    pub fn account_nonce(&self, account: &AccountId) -> u32 {
        self.system.account_nonce(account)
    }

    pub fn block_number(&self) -> BlockNumber {
        self.system.block_number()
    }

    pub fn total_issuance(&self) -> Balance {
        self.balances.total_issuance()
    }

    pub fn get_custom_data(&self, id: u32) -> Option<&custom_pallet::CustomData> {
        self.custom_pallet.get_data(id)
    }
}
```

#### **Configurações Auxiliares:**
```rust
// Configuração do Genesis
#[derive(Clone, Debug)]
pub struct GenesisConfig {
    pub balances: Vec<(AccountId, Balance)>,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            balances: vec![
                ("alice".to_string(), 1_000_000),
                ("bob".to_string(), 500_000),
                ("charlie".to_string(), 250_000),
            ],
        }
    }
}

// Eventos coletados de todos os pallets
#[derive(Clone, Debug)]
pub struct RuntimeEvents {
    pub system: Vec<system::Event<Runtime>>,
    pub balances: Vec<balances::Event<Runtime>>,
    pub custom_pallet: Vec<custom_pallet::Event<Runtime>>,
}

impl RuntimeEvents {
    pub fn total_events(&self) -> usize {
        self.system.len() + self.balances.len() + self.custom_pallet.len()
    }
}

// Helper para criar blocos
pub fn create_block(
    number: BlockNumber,
    parent_hash: Hash,
    extrinsics: Vec<Vec<u8>>,
) -> Block {
    Block {
        header: Header {
            number,
            parent_hash,
            state_root: [0u8; 32], // Simplificado
            extrinsics_root: [0u8; 32], // Simplificado
        },
        extrinsics,
    }
}

// Helper para criar extrinsics
pub fn create_transfer_extrinsic(from: &str, to: &str, amount: Balance) -> Vec<u8> {
    format!("transfer:{}:{}:{}", from, to, amount).into_bytes()
}

pub fn create_store_data_extrinsic(owner: &str, data: &str) -> Vec<u8> {
    format!("store_data:{}:{}", owner, data).into_bytes()
}
```

### Implementação de Teste

#### **Configuração de Teste:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_runtime() -> RuntimeInstance {
        let mut runtime = RuntimeInstance::new();
        let genesis = GenesisConfig::default();
        runtime.initialize_genesis(genesis);
        runtime
    }
}
```

### Testes Obrigatórios

#### **1. Inicialização do Runtime:**
```rust
#[test]
fn test_runtime_initialization() {
    let runtime = setup_runtime();
    
    // Verificar genesis
    assert_eq!(runtime.block_number(), 0);
    assert_eq!(runtime.account_balance(&"alice".to_string()), 1_000_000);
    assert_eq!(runtime.account_balance(&"bob".to_string()), 500_000);
    assert_eq!(runtime.total_issuance(), 1_750_000);
}
```

#### **2. Execução de Blocos:**
```rust
#[test]
fn test_block_execution() {
    let mut runtime = setup_runtime();
    
    // Criar bloco com transfer
    let extrinsics = vec![
        create_transfer_extrinsic("alice", "bob", 100_000),
    ];
    let block = create_block(1, [0u8; 32], extrinsics);
    
    let result = runtime.execute_block(block);
    assert_eq!(result, Ok(()));
    
    // Verificar estado após execução
    assert_eq!(runtime.block_number(), 1);
    assert_eq!(runtime.account_balance(&"alice".to_string()), 900_000);
    assert_eq!(runtime.account_balance(&"bob".to_string()), 600_000);
    assert_eq!(runtime.account_nonce(&"alice".to_string()), 1);
}
```

#### **3. Múltiplos Pallets:**
```rust
#[test]
fn test_multiple_pallets_interaction() {
    let mut runtime = setup_runtime();
    
    let extrinsics = vec![
        create_transfer_extrinsic("alice", "bob", 50_000),
        create_store_data_extrinsic("alice", "hello_world"),
    ];
    let block = create_block(1, [0u8; 32], extrinsics);
    
    runtime.execute_block(block).unwrap();
    
    // Verificar balances
    assert_eq!(runtime.account_balance(&"alice".to_string()), 950_000);
    assert_eq!(runtime.account_nonce(&"alice".to_string()), 2); // 2 extrinsics
    
    // Verificar custom data
    let data = runtime.get_custom_data(1).unwrap();
    assert_eq!(data.owner, "alice");
    assert_eq!(data.data, b"hello_world");
}
```

#### **4. Eventos do Runtime:**
```rust
#[test]
fn test_runtime_events() {
    let mut runtime = setup_runtime();
    
    let extrinsics = vec![
        create_transfer_extrinsic("alice", "charlie", 25_000),
        create_store_data_extrinsic("bob", "test_data"),
    ];
    let block = create_block(1, [0u8; 32], extrinsics);
    
    runtime.execute_block(block).unwrap();
    let events = runtime.collect_events();
    
    // Verificar eventos
    assert!(events.total_events() > 0);
    assert!(!events.balances.is_empty()); // Transfer event
    assert!(!events.custom_pallet.is_empty()); // DataStored event
    assert!(!events.system.is_empty()); // BlockFinalized event
}
```

#### **5. Sequência de Blocos:**
```rust
#[test]
fn test_block_sequence() {
    let mut runtime = setup_runtime();
    
    // Bloco 1
    let block1 = create_block(1, [0u8; 32], vec![
        create_transfer_extrinsic("alice", "bob", 100_000),
    ]);
    runtime.execute_block(block1).unwrap();
    
    // Bloco 2
    let block2 = create_block(2, [1u8; 32], vec![
        create_transfer_extrinsic("bob", "charlie", 50_000),
        create_store_data_extrinsic("charlie", "block2_data"),
    ]);
    runtime.execute_block(block2).unwrap();
    
    // Verificar estado final
    assert_eq!(runtime.block_number(), 2);
    assert_eq!(runtime.account_balance(&"alice".to_string()), 900_000);
    assert_eq!(runtime.account_balance(&"bob".to_string()), 550_000);
    assert_eq!(runtime.account_balance(&"charlie".to_string()), 300_000);
    
    // Verificar nonces
    assert_eq!(runtime.account_nonce(&"alice".to_string()), 1);
    assert_eq!(runtime.account_nonce(&"bob".to_string()), 1);
    assert_eq!(runtime.account_nonce(&"charlie".to_string()), 1);
}
```

#### **6. Validação de Extrinsics:**
```rust
#[test]
fn test_invalid_extrinsics() {
    let mut runtime = setup_runtime();
    
    // Transfer com saldo insuficiente
    let block = create_block(1, [0u8; 32], vec![
        create_transfer_extrinsic("alice", "bob", 2_000_000), // Mais que o saldo
    ]);
    
    let result = runtime.execute_block(block);
    assert!(result.is_err());
    
    // Estado não deve ter mudado
    assert_eq!(runtime.account_balance(&"alice".to_string()), 1_000_000);
    assert_eq!(runtime.account_nonce(&"alice".to_string()), 0);
}
```

### Critérios de Avaliação

- [ ] Runtime implementado com múltiplos pallets integrados
- [ ] System pallet gerencia contas e blocos corretamente
- [ ] Balances pallet executa transfers e mantém estado
- [ ] Custom pallet armazena dados com validações
- [ ] Genesis configuration inicializa estado corretamente
- [ ] Execução de blocos processa extrinsics sequencialmente
- [ ] Eventos são coletados de todos os pallets
- [ ] Runtime APIs fornecem consultas corretas
- [ ] Validação de extrinsics previne estados inválidos
- [ ] Todos os testes passam
- [ ] Código bem estruturado e documentado

### Contexto Teórico

**Substrate Runtime** é o coração de uma blockchain Substrate, definindo a lógica de negócio e as regras de transição de estado.

**Componentes Principais**:
- **Pallets**: Módulos que implementam funcionalidades específicas
- **Runtime Configuration**: Como pallets são configurados e conectados
- **Executive**: Coordena execução de blocos e extrinsics
- **APIs**: Interfaces para consultas externas

**Fluxo de Execução**:
1. **Genesis**: Estado inicial da blockchain
2. **Block Import**: Validação e execução de novos blocos
3. **Extrinsic Processing**: Execução de transações individuais
4. **State Transition**: Atualização do estado da blockchain
5. **Event Emission**: Notificação de mudanças de estado

**Integração de Pallets**:
- **Config Traits**: Definem dependências entre pallets
- **Event Aggregation**: Coleta eventos de todos os pallets
- **Cross-Pallet Calls**: Pallets podem chamar funções de outros
- **Shared Types**: Tipos comuns usados por múltiplos pallets

**Runtime APIs**:
- **Query Interface**: Consultas ao estado sem modificá-lo
- **Metadata**: Informações sobre estrutura do runtime
- **Version Info**: Controle de versão para upgrades

### Próximos Passos

Após completar este desafio:
1. Estude o Substrate Node Template real
2. Explore pallets avançados (Governance, Staking)
3. Pratique com runtime upgrades
4. Implemente pallets customizados complexos

### Recursos Adicionais

- [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
- [Runtime Development](https://docs.substrate.io/build/runtime-development/)
- [Pallet Integration](https://docs.substrate.io/build/custom-pallets/)
- [Runtime APIs](https://docs.substrate.io/build/custom-rpc/)
