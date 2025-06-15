## Desafio 11: Transaction Pool e Priorização

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você implementará um sistema simplificado de Transaction Pool que simula como transações são armazenadas, priorizadas e selecionadas para inclusão em blocos. O foco é entender os mecanismos de priorização, dependências entre transações e políticas de remoção.

### Conceitos Principais Abordados

1. **Transaction Pool**: Pool de transações pendentes aguardando inclusão em blocos
2. **Priorização**: Sistema de prioridades baseado em taxas e importância
3. **Dependências**: Transações que dependem de outras (nonce sequencial)
4. **Longevity**: Tempo de vida das transações no pool
5. **Block Building**: Seleção de transações para formar um bloco

### Estruturas a Implementar

#### **`TransactionHash`:**
```rust
pub type TransactionHash = [u8; 32];

// Helper para gerar hash simples
pub fn simple_hash(data: &[u8]) -> TransactionHash {
    let mut hash = [0u8; 32];
    for (i, byte) in data.iter().enumerate() {
        if i >= 32 { break; }
        hash[i] = *byte;
    }
    hash
}
```

#### **`Transaction` Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub hash: TransactionHash,
    pub sender: String, // AccountId simplificado
    pub nonce: u64,
    pub priority: u64,
    pub longevity: u64, // Blocos que a transação permanece válida
    pub requires: Vec<TransactionHash>, // Dependências
    pub provides: Vec<TransactionHash>, // O que esta transação fornece
    pub data: Vec<u8>, // Dados da transação
}

impl Transaction {
    pub fn new(
        sender: String,
        nonce: u64,
        priority: u64,
        longevity: u64,
        data: Vec<u8>,
    ) -> Self {
        let hash_input = format!("{}:{}:{}", sender, nonce, priority);
        let hash = simple_hash(hash_input.as_bytes());
        
        // Gerar provides baseado no sender e nonce
        let provides_input = format!("{}:{}", sender, nonce);
        let provides = vec![simple_hash(provides_input.as_bytes())];
        
        // Gerar requires baseado no nonce anterior (se > 0)
        let requires = if nonce > 0 {
            let requires_input = format!("{}:{}", sender, nonce - 1);
            vec![simple_hash(requires_input.as_bytes())]
        } else {
            Vec::new()
        };
        
        Self {
            hash,
            sender,
            nonce,
            priority,
            longevity,
            requires,
            provides,
            data,
        }
    }
}
```

#### **`PoolStatus` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum PoolStatus {
    Ready,    // Pronta para inclusão
    Future,   // Aguardando dependências
    Invalid,  // Inválida (será removida)
}
```

#### **`PoolTransaction` Struct:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct PoolTransaction {
    pub transaction: Transaction,
    pub status: PoolStatus,
    pub inserted_at: u64, // Bloco em que foi inserida
    pub retries: u32,     // Tentativas de inclusão
}

impl PoolTransaction {
    pub fn new(transaction: Transaction, current_block: u64) -> Self {
        Self {
            transaction,
            status: PoolStatus::Future, // Inicialmente Future, será promovida se possível
            inserted_at: current_block,
            retries: 0,
        }
    }
    
    pub fn is_expired(&self, current_block: u64) -> bool {
        current_block > self.inserted_at + self.transaction.longevity
    }
    
    pub fn can_be_included(&self) -> bool {
        matches!(self.status, PoolStatus::Ready)
    }
}
```

#### **`TransactionPool` Struct:**
```rust
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

pub struct TransactionPool {
    transactions: HashMap<TransactionHash, PoolTransaction>,
    ready_queue: BinaryHeap<ReadyTransaction>, // Heap para priorização
    provided_tags: HashSet<TransactionHash>,   // Tags fornecidas por transações ready
    current_block: u64,
    max_pool_size: usize,
}

// Wrapper para usar no BinaryHeap (max-heap por prioridade)
#[derive(Clone, Debug, PartialEq, Eq)]
struct ReadyTransaction {
    hash: TransactionHash,
    priority: u64,
    inserted_at: u64,
}

impl Ord for ReadyTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        // Primeiro por prioridade (maior é melhor)
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => {
                // Em caso de empate, mais antiga é melhor (FIFO)
                other.inserted_at.cmp(&self.inserted_at)
            }
            other => other,
        }
    }
}

impl PartialOrd for ReadyTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

### Métodos Obrigatórios do `TransactionPool`

#### **Construtor e Utilitários:**
```rust
impl TransactionPool {
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            ready_queue: BinaryHeap::new(),
            provided_tags: HashSet::new(),
            current_block: 0,
            max_pool_size,
        }
    }
    
    pub fn set_current_block(&mut self, block_number: u64) {
        self.current_block = block_number;
        self.cleanup_expired();
    }
    
    fn cleanup_expired(&mut self) {
        let current_block = self.current_block;
        let expired_hashes: Vec<TransactionHash> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| pool_tx.is_expired(current_block))
            .map(|(hash, _)| *hash)
            .collect();
        
        for hash in expired_hashes {
            self.remove_transaction(&hash);
        }
    }
}
```

#### **Inserção de Transações:**
```rust
impl TransactionPool {
    pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        // Verificar limite do pool
        if self.transactions.len() >= self.max_pool_size {
            return Err("Pool is full".to_string());
        }
        
        // Verificar se já existe
        if self.transactions.contains_key(&transaction.hash) {
            return Err("Transaction already in pool".to_string());
        }
        
        let pool_tx = PoolTransaction::new(transaction, self.current_block);
        let hash = pool_tx.transaction.hash;
        
        // Inserir no pool
        self.transactions.insert(hash, pool_tx);
        
        // Tentar promover para ready
        self.try_promote_transaction(&hash);
        
        Ok(())
    }
    
    fn try_promote_transaction(&mut self, hash: &TransactionHash) {
        if let Some(pool_tx) = self.transactions.get_mut(hash) {
            // Verificar se todas as dependências estão satisfeitas
            let dependencies_satisfied = pool_tx.transaction.requires
                .iter()
                .all(|req| self.provided_tags.contains(req));
            
            if dependencies_satisfied && matches!(pool_tx.status, PoolStatus::Future) {
                // Promover para Ready
                pool_tx.status = PoolStatus::Ready;
                
                // Adicionar à fila de prontas
                let ready_tx = ReadyTransaction {
                    hash: *hash,
                    priority: pool_tx.transaction.priority,
                    inserted_at: pool_tx.inserted_at,
                };
                self.ready_queue.push(ready_tx);
                
                // Adicionar tags fornecidas
                for tag in &pool_tx.transaction.provides {
                    self.provided_tags.insert(*tag);
                }
                
                // Tentar promover outras transações que dependem desta
                self.try_promote_dependent_transactions(hash);
            }
        }
    }
    
    fn try_promote_dependent_transactions(&mut self, promoted_hash: &TransactionHash) {
        let provided_tags: Vec<TransactionHash> = self.transactions
            .get(promoted_hash)
            .map(|tx| tx.transaction.provides.clone())
            .unwrap_or_default();
        
        let dependent_hashes: Vec<TransactionHash> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| {
                matches!(pool_tx.status, PoolStatus::Future) &&
                pool_tx.transaction.requires.iter().any(|req| provided_tags.contains(req))
            })
            .map(|(hash, _)| *hash)
            .collect();
        
        for hash in dependent_hashes {
            self.try_promote_transaction(&hash);
        }
    }
}
```

#### **Seleção para Blocos:**
```rust
impl TransactionPool {
    pub fn select_transactions_for_block(&mut self, max_transactions: usize) -> Vec<Transaction> {
        let mut selected = Vec::new();
        let mut temp_ready_queue = std::mem::take(&mut self.ready_queue);
        
        while selected.len() < max_transactions && !temp_ready_queue.is_empty() {
            if let Some(ready_tx) = temp_ready_queue.pop() {
                if let Some(pool_tx) = self.transactions.get(&ready_tx.hash) {
                    if pool_tx.can_be_included() {
                        selected.push(pool_tx.transaction.clone());
                        // Não remover ainda - será removido quando o bloco for finalizado
                    }
                }
            }
        }
        
        // Restaurar a fila (sem as transações selecionadas)
        self.ready_queue = temp_ready_queue;
        
        selected
    }
    
    pub fn finalize_block(&mut self, included_transactions: &[TransactionHash]) {
        for hash in included_transactions {
            self.remove_transaction(hash);
        }
        
        // Reconstruir ready_queue sem as transações removidas
        self.rebuild_ready_queue();
    }
    
    fn remove_transaction(&mut self, hash: &TransactionHash) {
        if let Some(pool_tx) = self.transactions.remove(hash) {
            // Remover tags fornecidas
            for tag in &pool_tx.transaction.provides {
                self.provided_tags.remove(tag);
            }
            
            // Rebaixar transações dependentes para Future
            self.demote_dependent_transactions(&pool_tx.transaction.provides);
        }
    }
    
    fn demote_dependent_transactions(&mut self, removed_provides: &[TransactionHash]) {
        let dependent_hashes: Vec<TransactionHash> = self.transactions
            .iter()
            .filter(|(_, pool_tx)| {
                matches!(pool_tx.status, PoolStatus::Ready) &&
                pool_tx.transaction.requires.iter().any(|req| removed_provides.contains(req))
            })
            .map(|(hash, _)| *hash)
            .collect();
        
        for hash in dependent_hashes {
            if let Some(pool_tx) = self.transactions.get_mut(&hash) {
                pool_tx.status = PoolStatus::Future;
                
                // Remover tags fornecidas por esta transação
                for tag in &pool_tx.transaction.provides {
                    self.provided_tags.remove(tag);
                }
            }
        }
    }
    
    fn rebuild_ready_queue(&mut self) {
        self.ready_queue.clear();
        
        for (hash, pool_tx) in &self.transactions {
            if matches!(pool_tx.status, PoolStatus::Ready) {
                let ready_tx = ReadyTransaction {
                    hash: *hash,
                    priority: pool_tx.transaction.priority,
                    inserted_at: pool_tx.inserted_at,
                };
                self.ready_queue.push(ready_tx);
            }
        }
    }
}
```

#### **Métodos de Consulta:**
```rust
impl TransactionPool {
    pub fn pool_size(&self) -> usize {
        self.transactions.len()
    }
    
    pub fn ready_count(&self) -> usize {
        self.ready_queue.len()
    }
    
    pub fn future_count(&self) -> usize {
        self.transactions.values()
            .filter(|tx| matches!(tx.status, PoolStatus::Future))
            .count()
    }
    
    pub fn get_transaction(&self, hash: &TransactionHash) -> Option<&PoolTransaction> {
        self.transactions.get(hash)
    }
    
    pub fn contains_transaction(&self, hash: &TransactionHash) -> bool {
        self.transactions.contains_key(hash)
    }
    
    pub fn get_ready_transactions(&self) -> Vec<&Transaction> {
        self.ready_queue
            .iter()
            .filter_map(|ready_tx| {
                self.transactions.get(&ready_tx.hash)
                    .map(|pool_tx| &pool_tx.transaction)
            })
            .collect()
    }
}
```

### Implementação de Teste

#### **Configuração de Teste:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_transaction(
        sender: &str,
        nonce: u64,
        priority: u64,
        data: &str,
    ) -> Transaction {
        Transaction::new(
            sender.to_string(),
            nonce,
            priority,
            10, // longevity
            data.as_bytes().to_vec(),
        )
    }
}
```

### Testes Obrigatórios

#### **1. Inserção Básica:**
```rust
#[test]
fn test_submit_transaction() {
    let mut pool = TransactionPool::new(100);
    let tx = create_test_transaction("alice", 0, 100, "transfer");
    
    let result = pool.submit_transaction(tx.clone());
    assert_eq!(result, Ok(()));
    assert_eq!(pool.pool_size(), 1);
    assert!(pool.contains_transaction(&tx.hash));
}

#[test]
fn test_submit_duplicate_transaction() {
    let mut pool = TransactionPool::new(100);
    let tx = create_test_transaction("alice", 0, 100, "transfer");
    
    pool.submit_transaction(tx.clone()).unwrap();
    let result = pool.submit_transaction(tx);
    assert!(result.is_err());
    assert_eq!(pool.pool_size(), 1);
}

#[test]
fn test_pool_size_limit() {
    let mut pool = TransactionPool::new(2);
    
    let tx1 = create_test_transaction("alice", 0, 100, "tx1");
    let tx2 = create_test_transaction("bob", 0, 100, "tx2");
    let tx3 = create_test_transaction("charlie", 0, 100, "tx3");
    
    assert_eq!(pool.submit_transaction(tx1), Ok(()));
    assert_eq!(pool.submit_transaction(tx2), Ok(()));
    assert!(pool.submit_transaction(tx3).is_err());
    assert_eq!(pool.pool_size(), 2);
}
```

#### **2. Priorização:**
```rust
#[test]
fn test_transaction_prioritization() {
    let mut pool = TransactionPool::new(100);
    
    // Inserir transações com prioridades diferentes
    let tx_low = create_test_transaction("alice", 0, 50, "low_priority");
    let tx_high = create_test_transaction("bob", 0, 200, "high_priority");
    let tx_medium = create_test_transaction("charlie", 0, 100, "medium_priority");
    
    pool.submit_transaction(tx_low).unwrap();
    pool.submit_transaction(tx_high.clone()).unwrap();
    pool.submit_transaction(tx_medium).unwrap();
    
    // Selecionar transações - deve vir em ordem de prioridade
    let selected = pool.select_transactions_for_block(3);
    assert_eq!(selected.len(), 3);
    assert_eq!(selected[0].hash, tx_high.hash); // Maior prioridade primeiro
}
```

#### **3. Dependências (Nonce):**
```rust
#[test]
fn test_nonce_dependencies() {
    let mut pool = TransactionPool::new(100);
    
    // Inserir transações fora de ordem
    let tx2 = create_test_transaction("alice", 2, 100, "tx2");
    let tx1 = create_test_transaction("alice", 1, 100, "tx1");
    let tx0 = create_test_transaction("alice", 0, 100, "tx0");
    
    pool.submit_transaction(tx2.clone()).unwrap();
    pool.submit_transaction(tx1.clone()).unwrap();
    pool.submit_transaction(tx0.clone()).unwrap();
    
    // Apenas tx0 deve estar ready inicialmente
    assert_eq!(pool.ready_count(), 1);
    assert_eq!(pool.future_count(), 2);
    
    let ready_txs = pool.get_ready_transactions();
    assert_eq!(ready_txs[0].hash, tx0.hash);
}

#[test]
fn test_dependency_chain_promotion() {
    let mut pool = TransactionPool::new(100);
    
    let tx0 = create_test_transaction("alice", 0, 100, "tx0");
    let tx1 = create_test_transaction("alice", 1, 100, "tx1");
    let tx2 = create_test_transaction("alice", 2, 100, "tx2");
    
    // Inserir em ordem reversa
    pool.submit_transaction(tx2.clone()).unwrap();
    pool.submit_transaction(tx1.clone()).unwrap();
    pool.submit_transaction(tx0.clone()).unwrap();
    
    // Todas devem ser promovidas em cascata
    assert_eq!(pool.ready_count(), 3);
    assert_eq!(pool.future_count(), 0);
}
```

#### **4. Seleção para Bloco:**
```rust
#[test]
fn test_block_selection() {
    let mut pool = TransactionPool::new(100);
    
    let tx1 = create_test_transaction("alice", 0, 100, "tx1");
    let tx2 = create_test_transaction("bob", 0, 200, "tx2");
    let tx3 = create_test_transaction("charlie", 0, 50, "tx3");
    
    pool.submit_transaction(tx1).unwrap();
    pool.submit_transaction(tx2.clone()).unwrap();
    pool.submit_transaction(tx3).unwrap();
    
    // Selecionar apenas 2 transações
    let selected = pool.select_transactions_for_block(2);
    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].hash, tx2.hash); // Maior prioridade
    
    // Pool ainda deve conter todas as transações
    assert_eq!(pool.pool_size(), 3);
}

#[test]
fn test_block_finalization() {
    let mut pool = TransactionPool::new(100);
    
    let tx1 = create_test_transaction("alice", 0, 100, "tx1");
    let tx2 = create_test_transaction("alice", 1, 100, "tx2");
    
    pool.submit_transaction(tx1.clone()).unwrap();
    pool.submit_transaction(tx2.clone()).unwrap();
    
    assert_eq!(pool.ready_count(), 2);
    
    // Finalizar bloco com tx1
    pool.finalize_block(&[tx1.hash]);
    
    // tx1 deve ser removido, tx2 deve ser rebaixado para Future
    assert_eq!(pool.pool_size(), 1);
    assert_eq!(pool.ready_count(), 0);
    assert_eq!(pool.future_count(), 1);
}
```

#### **5. Expiração:**
```rust
#[test]
fn test_transaction_expiration() {
    let mut pool = TransactionPool::new(100);
    
    let tx = Transaction::new(
        "alice".to_string(),
        0,
        100,
        5, // longevity = 5 blocos
        b"data".to_vec(),
    );
    
    pool.submit_transaction(tx.clone()).unwrap();
    assert_eq!(pool.pool_size(), 1);
    
    // Avançar 6 blocos (além da longevity)
    pool.set_current_block(6);
    
    // Transação deve ter sido removida
    assert_eq!(pool.pool_size(), 0);
}
```

### Critérios de Avaliação

- [ ] `TransactionPool` implementado com todas as funcionalidades
- [ ] Sistema de priorização funciona corretamente
- [ ] Dependências entre transações (nonce) são respeitadas
- [ ] Promoção automática de transações Future para Ready
- [ ] Seleção de transações para blocos por prioridade
- [ ] Finalização de blocos remove transações incluídas
- [ ] Limpeza automática de transações expiradas
- [ ] Limite de tamanho do pool é respeitado
- [ ] Todos os testes passam
- [ ] Código bem estruturado e documentado

### Contexto Teórico

**Transaction Pool** é um componente crucial em blockchains que gerencia transações pendentes antes de serem incluídas em blocos.

**Características Principais**:
- **Armazenamento Temporário**: Mantém transações até serem incluídas ou expiradas
- **Priorização**: Ordena transações por importância (taxas, prioridade)
- **Validação**: Verifica se transações são válidas antes da inclusão
- **Dependências**: Gerencia ordem de execução (nonces sequenciais)

**Estados das Transações**:
- **Ready**: Pronta para inclusão imediata
- **Future**: Aguardando dependências serem satisfeitas
- **Invalid**: Inválida e será removida

**Algoritmos de Seleção**:
- **Priority-based**: Maior prioridade/taxa primeiro
- **FIFO**: Primeira a entrar, primeira a sair (em caso de empate)
- **Dependency-aware**: Respeita ordem de dependências

**Políticas de Remoção**:
- **Longevity**: Remove transações muito antigas
- **Pool Limits**: Remove transações de menor prioridade quando pool está cheio
- **Invalid State**: Remove transações que se tornaram inválidas

### Próximos Passos

Após completar este desafio:
1. Estude implementações reais de transaction pools
2. Explore algoritmos avançados de priorização
3. Pratique com cenários de alta concorrência

### Recursos Adicionais

- [Substrate Transaction Pool](https://docs.substrate.io/learn/transaction-pool/)
- [Transaction Validity](https://docs.substrate.io/build/transaction-format/)
- [Priority and Longevity](https://docs.substrate.io/build/tx-weights-fees/)
