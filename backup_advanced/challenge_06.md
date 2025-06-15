## Desafio 6: RPC Customizado Simples para Pallet de Contador

**Nível de Dificuldade:** Avançado

### Descrição do Objetivo

Neste desafio, você criará uma interface RPC (Remote Procedure Call) simples para consultar dados de um pallet de contador básico. O objetivo é entender como expor funcionalidades de leitura de um pallet através de endpoints RPC customizados.

**Conceitos Principais Abordados:**
1. **Definição de API RPC** através de traits Rust
2. **Implementação de handlers RPC** para acessar dados do pallet
3. **Separação entre lógica de negócio e interface de consulta**
4. **Tratamento de erros em APIs RPC**

### Estruturas a Implementar:

#### **1. Pallet de Contador Simplificado:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct CounterPallet {
    value: u32,
    last_updated_by: Option<String>, // AccountId simplificado como String
}

impl CounterPallet {
    pub fn new() -> Self {
        Self { 
            value: 0, 
            last_updated_by: None 
        }
    }

    pub fn increment(&mut self, who: String) {
        self.value += 1;
        self.last_updated_by = Some(who);
    }

    pub fn set_value(&mut self, who: String, new_value: u32) {
        self.value = new_value;
        self.last_updated_by = Some(who);
    }

    // Métodos de acesso para RPC
    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn get_last_updated_by(&self) -> Option<String> {
        self.last_updated_by.clone()
    }
}
```

#### **2. Tipos de Erro RPC:**
```rust
#[derive(Debug, PartialEq)]
pub enum RpcError {
    PalletNotInitialized,
    ValueNotFound,
    InternalError(String),
}
```

#### **3. Trait da API RPC:**
```rust
pub trait CounterRpcApi {
    /// Retorna o valor atual do contador
    fn get_current_value(&self) -> Result<u32, RpcError>;
    
    /// Retorna quem foi o último a atualizar o contador
    fn get_last_updater(&self) -> Result<Option<String>, RpcError>;
    
    /// Retorna informações resumidas do contador
    fn get_counter_info(&self) -> Result<CounterInfo, RpcError>;
}
```

#### **4. Estrutura de Resposta:**
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct CounterInfo {
    pub current_value: u32,
    pub last_updated_by: Option<String>,
    pub is_initialized: bool,
}
```

#### **5. Handler RPC:**
```rust
pub struct CounterRpcHandler<'a> {
    pallet: &'a CounterPallet,
}

impl<'a> CounterRpcHandler<'a> {
    pub fn new(pallet: &'a CounterPallet) -> Self {
        Self { pallet }
    }
}

impl<'a> CounterRpcApi for CounterRpcHandler<'a> {
    fn get_current_value(&self) -> Result<u32, RpcError> {
        Ok(self.pallet.get_value())
    }

    fn get_last_updater(&self) -> Result<Option<String>, RpcError> {
        Ok(self.pallet.get_last_updated_by())
    }

    fn get_counter_info(&self) -> Result<CounterInfo, RpcError> {
        Ok(CounterInfo {
            current_value: self.pallet.get_value(),
            last_updated_by: self.pallet.get_last_updated_by(),
            is_initialized: self.pallet.get_last_updated_by().is_some(),
        })
    }
}
```

### Testes Obrigatórios

Implemente os seguintes testes:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_value() {
        let mut pallet = CounterPallet::new();
        pallet.set_value("alice".to_string(), 42);
        
        let rpc_handler = CounterRpcHandler::new(&pallet);
        assert_eq!(rpc_handler.get_current_value(), Ok(42));
    }

    #[test]
    fn test_get_last_updater() {
        let mut pallet = CounterPallet::new();
        pallet.increment("bob".to_string());
        
        let rpc_handler = CounterRpcHandler::new(&pallet);
        assert_eq!(rpc_handler.get_last_updater(), Ok(Some("bob".to_string())));
    }

    #[test]
    fn test_get_counter_info() {
        let mut pallet = CounterPallet::new();
        pallet.set_value("charlie".to_string(), 100);
        
        let rpc_handler = CounterRpcHandler::new(&pallet);
        let info = rpc_handler.get_counter_info().unwrap();
        
        assert_eq!(info.current_value, 100);
        assert_eq!(info.last_updated_by, Some("charlie".to_string()));
        assert_eq!(info.is_initialized, true);
    }

    #[test]
    fn test_uninitialized_counter() {
        let pallet = CounterPallet::new();
        let rpc_handler = CounterRpcHandler::new(&pallet);
        
        let info = rpc_handler.get_counter_info().unwrap();
        assert_eq!(info.current_value, 0);
        assert_eq!(info.last_updated_by, None);
        assert_eq!(info.is_initialized, false);
    }
}
```

### Critérios de Avaliação

- [ ] Pallet de contador implementado corretamente
- [ ] Trait `CounterRpcApi` definida com os métodos especificados
- [ ] Handler RPC implementa a trait corretamente
- [ ] Todos os testes passam
- [ ] Código está bem estruturado e documentado

### Contexto Teórico

**RPC (Remote Procedure Call)** permite que clientes externos consultem dados da blockchain sem precisar conhecer os detalhes internos do pallet. No Substrate:

- **JSON-RPC**: Protocolo padrão usado pelos nós Substrate
- **Endpoints Customizados**: Pallets podem expor suas próprias APIs RPC
- **Read-Only**: RPCs são tipicamente para consultas, não para modificar estado
- **Separação de Responsabilidades**: RPC handlers acessam dados do pallet mas não contêm lógica de negócio

### Próximos Passos

Após completar este desafio:
1. Estude a documentação oficial do Substrate sobre RPC customizado
2. Explore como implementar RPC com `jsonrpsee` em projetos reais
3. Pratique com endpoints RPC mais complexos que retornam dados estruturados

### Recursos Adicionais

- [Substrate RPC Documentation](https://docs.substrate.io/build/custom-rpc/)
- [jsonrpsee Crate](https://docs.rs/jsonrpsee/)
- [Substrate Node Template RPC Examples](https://github.com/substrate-developer-hub/substrate-node-template)