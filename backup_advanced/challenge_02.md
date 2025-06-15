## Desafio 2: Off-chain Worker Simples para Cotação de Preços

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você implementará um pallet que simula a funcionalidade básica de um Off-chain Worker (OCW). O OCW buscará um preço de um ativo de uma "fonte externa" simulada e submeterá esse preço à blockchain. O foco é entender a separação entre lógica off-chain (busca de dados) e on-chain (armazenamento).

### Conceitos Principais Abordados

1. **Lógica On-chain vs Off-chain**: Separação clara entre coleta de dados externa e armazenamento na blockchain
2. **Traits para Dependências Externas**: Simulação de APIs externas através de traits
3. **Autorização de Submissão**: Apenas contas autorizadas podem submeter dados
4. **Fluxo do OCW**: Worker busca dados → valida → submete transação
5. **Tratamento de Erros**: Falhas na busca externa vs falhas na submissão

### Estruturas a Implementar

#### **Trait para Fonte Externa:**
```rust
pub trait ExternalPriceProvider {
    fn fetch_price(&self) -> Result<u64, String>;
}
```

#### **`Origin` Enum:**
```rust
#[derive(Clone, PartialEq, Debug)]
pub enum Origin {
    Signed(String), // AccountId simplificado como String
}
```

#### **`Event` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    PriceUpdated { 
        new_price: u64, 
        submitted_by: String 
    },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    OffchainPriceFetchFailed,
    InvalidPriceSubmitter,
    BadOrigin,
}
```

#### **`Pallet` Struct:**
```rust
pub struct Pallet {
    current_price: Option<u64>,
    authorized_signer: String, // Conta autorizada a submeter preços
    emitted_events: Vec<Event>,
}
```

### Métodos Obrigatórios do `Pallet`

#### **Construtor:**
```rust
pub fn new(authorized_signer: String) -> Self {
    Self {
        current_price: None,
        authorized_signer,
        emitted_events: Vec::new(),
    }
}
```

#### **Métodos de Acesso:**
```rust
pub fn get_current_price(&self) -> Option<u64>
pub fn take_events(&mut self) -> Vec<Event>
fn deposit_event(&mut self, event: Event)
```

#### **Submissão On-chain:**
```rust
pub fn submit_price_onchain(
    &mut self, 
    origin: Origin, 
    price: u64
) -> Result<(), Error> {
    // 1. Verificar se origin é Signed
    // 2. Verificar se o signatário é autorizado
    // 3. Atualizar current_price
    // 4. Emitir evento PriceUpdated
}
```

#### **Simulação do Off-chain Worker:**
```rust
pub fn run_offchain_worker(
    &mut self, 
    price_provider: &impl ExternalPriceProvider
) -> Result<(), Error> {
    // 1. Buscar preço usando price_provider.fetch_price()
    // 2. Se sucesso, chamar submit_price_onchain com signer autorizado
    // 3. Se falha, retornar OffchainPriceFetchFailed
}
```

### Implementação de Teste

#### **Mock Price Provider:**
```rust
pub struct MockPriceProvider {
    should_succeed: bool,
    price_to_return: u64,
}

impl MockPriceProvider {
    pub fn new_success(price: u64) -> Self {
        Self { should_succeed: true, price_to_return: price }
    }
    
    pub fn new_failure() -> Self {
        Self { should_succeed: false, price_to_return: 0 }
    }
}

impl ExternalPriceProvider for MockPriceProvider {
    fn fetch_price(&self) -> Result<u64, String> {
        if self.should_succeed {
            Ok(self.price_to_return)
        } else {
            Err("Network error".to_string())
        }
    }
}
```

### Testes Obrigatórios

#### **1. Fluxo de Sucesso do OCW:**
```rust
#[test]
fn test_successful_ocw_flow() {
    let mut pallet = Pallet::new("authorized_account".to_string());
    let provider = MockPriceProvider::new_success(100);
    
    // OCW deve buscar preço e submeter com sucesso
    assert_eq!(pallet.run_offchain_worker(&provider), Ok(()));
    assert_eq!(pallet.get_current_price(), Some(100));
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::PriceUpdated { 
        new_price: 100, 
        submitted_by: "authorized_account".to_string() 
    });
}
```

#### **2. Falha na Busca de Preço:**
```rust
#[test]
fn test_ocw_fetch_failure() {
    let mut pallet = Pallet::new("authorized_account".to_string());
    let provider = MockPriceProvider::new_failure();
    
    // OCW deve falhar ao buscar preço
    assert_eq!(pallet.run_offchain_worker(&provider), Err(Error::OffchainPriceFetchFailed));
    assert_eq!(pallet.get_current_price(), None);
    assert_eq!(pallet.take_events().len(), 0);
}
```

#### **3. Submissão com Signatário Inválido:**
```rust
#[test]
fn test_invalid_submitter() {
    let mut pallet = Pallet::new("authorized_account".to_string());
    
    // Tentar submeter com conta não autorizada
    let result = pallet.submit_price_onchain(
        Origin::Signed("unauthorized_account".to_string()), 
        50
    );
    
    assert_eq!(result, Err(Error::InvalidPriceSubmitter));
    assert_eq!(pallet.get_current_price(), None);
}
```

#### **4. Submissão Direta Bem-sucedida:**
```rust
#[test]
fn test_direct_submission_success() {
    let mut pallet = Pallet::new("authorized_account".to_string());
    
    // Submissão direta com conta autorizada
    let result = pallet.submit_price_onchain(
        Origin::Signed("authorized_account".to_string()), 
        75
    );
    
    assert_eq!(result, Ok(()));
    assert_eq!(pallet.get_current_price(), Some(75));
}
```

### Critérios de Avaliação

- [ ] Pallet implementado com todos os métodos obrigatórios
- [ ] `ExternalPriceProvider` trait definida corretamente
- [ ] `MockPriceProvider` implementa a trait para testes
- [ ] Verificação de autorização funciona corretamente
- [ ] Separação clara entre lógica off-chain e on-chain
- [ ] Todos os testes passam
- [ ] Tratamento adequado de erros
- [ ] Código bem estruturado e documentado

### Contexto Teórico

**Off-chain Workers (OCWs)** são uma funcionalidade do Substrate que permite aos nós executar tarefas que requerem acesso externo (APIs, arquivos, etc.) sem afetar o processo de produção de blocos.

**Características dos OCWs:**
- **Execução Off-chain**: Processamento não está sujeito ao consenso
- **Acesso Externo**: Podem fazer chamadas HTTP, ler arquivos, etc.
- **Submissão de Transações**: Podem enviar transações de volta à blockchain
- **Não Determinísticos**: Diferentes nós podem ter resultados diferentes

**Casos de Uso Comuns:**
- Oráculos de preços
- Agregação de dados externos
- Computação intensiva
- Interação com APIs externas

**Segurança**: É crucial que apenas OCWs autorizados possam submeter dados críticos. A verificação de `authorized_signer` simula esse controle de acesso.

### Próximos Passos

Após completar este desafio:
1. Estude a documentação oficial sobre Off-chain Workers
2. Explore exemplos de oráculos no ecossistema Substrate
3. Pratique com chamadas HTTP reais em OCWs

### Recursos Adicionais

- [Substrate Off-chain Workers](https://docs.substrate.io/learn/offchain-operations/)
- [OCW Examples](https://github.com/substrate-developer-hub/recipes/tree/main/pallets/ocw)
- [Oracle Pallet Template](https://github.com/substrate-developer-hub/substrate-how-to-guides/tree/main/example-code/offchain-workers)