## Desafio 8: Pallet com Origin Customizado Simples

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você criará um pallet que gerencia uma lista de "membros especiais" e implementará um tipo de `Origin` customizado. O pallet terá uma funcionalidade específica que só pode ser executada por esses membros especiais, demonstrando como verificar autorização através de origins customizados.

### Conceitos Principais Abordados

1. **Origins Customizados**: Definição de tipos de origem além dos padrões (`Root`, `Signed`)
2. **Gerenciamento de Privilégios**: Sistema de membros especiais com permissões específicas
3. **Verificação de Autorização**: Validação de origins customizados antes da execução
4. **Pattern Matching**: Tratamento de diferentes tipos de origem
5. **HashSet para Storage**: Simulação eficiente de storage para conjuntos

### Estruturas a Implementar

#### **`SystemOrigin<AccountId>` Enum:**
```rust
#[derive(Clone, PartialEq, Debug)]
pub enum SystemOrigin<AccountId> {
    Root,
    Signed(AccountId),
}
```

#### **`MyCustomOrigin<AccountId>` Enum:**
```rust
#[derive(Clone, PartialEq, Debug)]
pub enum MyCustomOrigin<AccountId> {
    System(SystemOrigin<AccountId>),
    SpecialMember(AccountId), // Nosso Origin customizado
}
```

#### **`Config` Trait:**
```rust
pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug + Eq + core::hash::Hash;
}
```

#### **`Event<T: Config>` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Event<T: Config> {
    SpecialMemberAdded { who: T::AccountId },
    SpecialMemberRemoved { who: T::AccountId },
    SpecialActionExecuted { by_member: T::AccountId },
}
```

#### **`Error` Enum:**
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RequiresRoot,
    NotSpecialMember,
    MemberAlreadySpecial,
    MemberIsNotSpecial,
    InvalidOriginForSpecialAction,
}
```

#### **`Pallet<T: Config>` Struct:**
```rust
use std::collections::HashSet;

pub struct Pallet<T: Config> {
    special_members: HashSet<T::AccountId>,
    emitted_events: Vec<Event<T>>,
}
```

### Métodos Obrigatórios do `Pallet<T: Config>`

#### **Construtor e Utilitários:**
```rust
pub fn new() -> Self {
    Self {
        special_members: HashSet::new(),
        emitted_events: Vec::new(),
    }
}

fn deposit_event(&mut self, event: Event<T>) {
    self.emitted_events.push(event);
}

pub fn take_events(&mut self) -> Vec<Event<T>> {
    std::mem::take(&mut self.emitted_events)
}

fn ensure_root(origin: SystemOrigin<T::AccountId>) -> Result<(), Error> {
    match origin {
        SystemOrigin::Root => Ok(()),
        _ => Err(Error::RequiresRoot),
    }
}
```

#### **Gerenciamento de Membros Especiais:**
```rust
pub fn add_special_member(
    &mut self, 
    origin: SystemOrigin<T::AccountId>, 
    member_to_add: T::AccountId
) -> Result<(), Error> {
    Self::ensure_root(origin)?;
    
    if self.special_members.contains(&member_to_add) {
        return Err(Error::MemberAlreadySpecial);
    }
    
    self.special_members.insert(member_to_add.clone());
    self.deposit_event(Event::SpecialMemberAdded { who: member_to_add });
    Ok(())
}

pub fn remove_special_member(
    &mut self, 
    origin: SystemOrigin<T::AccountId>, 
    member_to_remove: T::AccountId
) -> Result<(), Error> {
    Self::ensure_root(origin)?;
    
    if !self.special_members.contains(&member_to_remove) {
        return Err(Error::MemberIsNotSpecial);
    }
    
    self.special_members.remove(&member_to_remove);
    self.deposit_event(Event::SpecialMemberRemoved { who: member_to_remove });
    Ok(())
}
```

#### **Ação Especial com Origin Customizado:**
```rust
pub fn execute_special_action(
    &mut self, 
    origin: MyCustomOrigin<T::AccountId>
) -> Result<(), Error> {
    // Verificar se é SpecialMember origin
    let who = match origin {
        MyCustomOrigin::SpecialMember(account) => account,
        _ => return Err(Error::InvalidOriginForSpecialAction),
    };
    
    // Verificar se é realmente um membro especial
    if !self.special_members.contains(&who) {
        return Err(Error::NotSpecialMember);
    }
    
    // Executar ação especial (neste caso, apenas emitir evento)
    self.deposit_event(Event::SpecialActionExecuted { by_member: who });
    Ok(())
}
```

#### **Métodos de Consulta (para testes):**
```rust
pub fn is_special_member(&self, account: &T::AccountId) -> bool {
    self.special_members.contains(account)
}

pub fn special_members_count(&self) -> usize {
    self.special_members.len()
}
```

### Implementação de Teste

#### **Configuração de Teste:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    type TestAccountId = u32;

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = TestAccountId;
    }

    type TestPallet = Pallet<TestConfig>;
}
```

### Testes Obrigatórios

#### **1. Gerenciamento de Membros Especiais:**
```rust
#[test]
fn test_add_special_member_success() {
    let mut pallet = TestPallet::new();
    
    let result = pallet.add_special_member(SystemOrigin::Root, 1);
    assert_eq!(result, Ok(()));
    assert!(pallet.is_special_member(&1));
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::SpecialMemberAdded { who: 1 });
}

#[test]
fn test_add_member_already_special() {
    let mut pallet = TestPallet::new();
    
    // Adicionar membro
    pallet.add_special_member(SystemOrigin::Root, 1).unwrap();
    
    // Tentar adicionar novamente
    let result = pallet.add_special_member(SystemOrigin::Root, 1);
    assert_eq!(result, Err(Error::MemberAlreadySpecial));
}

#[test]
fn test_add_member_requires_root() {
    let mut pallet = TestPallet::new();
    
    let result = pallet.add_special_member(SystemOrigin::Signed(1), 2);
    assert_eq!(result, Err(Error::RequiresRoot));
    assert!(!pallet.is_special_member(&2));
}
```

#### **2. Remoção de Membros:**
```rust
#[test]
fn test_remove_special_member_success() {
    let mut pallet = TestPallet::new();
    
    // Primeiro adicionar
    pallet.add_special_member(SystemOrigin::Root, 1).unwrap();
    pallet.take_events(); // Limpar eventos
    
    // Depois remover
    let result = pallet.remove_special_member(SystemOrigin::Root, 1);
    assert_eq!(result, Ok(()));
    assert!(!pallet.is_special_member(&1));
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::SpecialMemberRemoved { who: 1 });
}

#[test]
fn test_remove_nonexistent_member() {
    let mut pallet = TestPallet::new();
    
    let result = pallet.remove_special_member(SystemOrigin::Root, 1);
    assert_eq!(result, Err(Error::MemberIsNotSpecial));
}
```

#### **3. Ação Especial com Origin Customizado:**
```rust
#[test]
fn test_special_action_success() {
    let mut pallet = TestPallet::new();
    
    // Adicionar membro especial
    pallet.add_special_member(SystemOrigin::Root, 1).unwrap();
    pallet.take_events(); // Limpar eventos
    
    // Executar ação especial
    let result = pallet.execute_special_action(
        MyCustomOrigin::SpecialMember(1)
    );
    assert_eq!(result, Ok(()));
    
    let events = pallet.take_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], Event::SpecialActionExecuted { by_member: 1 });
}

#[test]
fn test_special_action_not_member() {
    let mut pallet = TestPallet::new();
    
    // Tentar executar ação sem ser membro especial
    let result = pallet.execute_special_action(
        MyCustomOrigin::SpecialMember(1)
    );
    assert_eq!(result, Err(Error::NotSpecialMember));
}

#[test]
fn test_special_action_invalid_origin() {
    let mut pallet = TestPallet::new();
    
    // Tentar com origin System
    let result = pallet.execute_special_action(
        MyCustomOrigin::System(SystemOrigin::Root)
    );
    assert_eq!(result, Err(Error::InvalidOriginForSpecialAction));
    
    let result = pallet.execute_special_action(
        MyCustomOrigin::System(SystemOrigin::Signed(1))
    );
    assert_eq!(result, Err(Error::InvalidOriginForSpecialAction));
}
```

#### **4. Testes de Estado:**
```rust
#[test]
fn test_multiple_members() {
    let mut pallet = TestPallet::new();
    
    // Adicionar múltiplos membros
    pallet.add_special_member(SystemOrigin::Root, 1).unwrap();
    pallet.add_special_member(SystemOrigin::Root, 2).unwrap();
    pallet.add_special_member(SystemOrigin::Root, 3).unwrap();
    
    assert_eq!(pallet.special_members_count(), 3);
    assert!(pallet.is_special_member(&1));
    assert!(pallet.is_special_member(&2));
    assert!(pallet.is_special_member(&3));
    assert!(!pallet.is_special_member(&4));
}
```

### Critérios de Avaliação

- [ ] `SystemOrigin` e `MyCustomOrigin` enums definidos corretamente
- [ ] Pallet gerencia HashSet de membros especiais
- [ ] Função `add_special_member` com verificação Root
- [ ] Função `remove_special_member` com verificação Root
- [ ] Função `execute_special_action` com origin customizado
- [ ] Verificação adequada de permissões
- [ ] Tratamento correto de todos os casos de erro
- [ ] Eventos emitidos corretamente
- [ ] Todos os testes passam
- [ ] Código bem estruturado e documentado

### Contexto Teórico

**`Origin` no Substrate/FRAME** representa a entidade que iniciou uma transação. Os tipos mais comuns são:
- `frame_system::RawOrigin::Root`: Superusuário com privilégios máximos
- `frame_system::RawOrigin::Signed(AccountId)`: Conta específica que assinou a transação

**Custom Origins** permitem definir tipos de autorização mais específicos:
- Membros de um coletivo
- Contas com roles específicos
- Combinações de múltiplas assinaturas
- Condições temporais ou contextuais

**`EnsureOrigin` Trait** é usada em pallets reais para validar origins:
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn special_function(
        origin: OriginFor<T>,
        // parâmetros
    ) -> DispatchResult {
        let who = ensure_special_member(origin)?;
        // lógica da função
    }
}
```

**Casos de Uso Reais**:
- **Democracy Pallet**: Origins para diferentes tipos de propostas
- **Collective Pallet**: Origins baseados em votação de membros
- **Sudo Pallet**: Origin Root para desenvolvimento
- **Multisig Pallet**: Origins baseados em múltiplas assinaturas

### Próximos Passos

Após completar este desafio:
1. Estude implementações de `EnsureOrigin` em pallets reais
2. Explore o sistema de origins do Democracy e Collective pallets
3. Pratique com origins mais complexos (threshold, temporal, etc.)

### Recursos Adicionais

- [Substrate Origins Documentation](https://docs.substrate.io/build/origins/)
- [Custom Origins Examples](https://github.com/paritytech/substrate/tree/master/frame/collective)
- [EnsureOrigin Implementations](https://docs.rs/frame-support/latest/frame_support/traits/trait.EnsureOrigin.html)
