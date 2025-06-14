## Desafio 2: Pallet Simplificado de Staking por Era

**Nível de Dificuldade:** Avançado

**Descrição do Objetivo:**

Você implementará a lógica de um "pallet" simplificado que permite aos usuários "stakarem" (bloquearem) uma quantidade de seus tokens por uma "era" específica. Os usuários só poderão resgatar (unstake) tokens de eras que já passaram. O pallet também simulará um sistema de saldos para as contas.

**Requisitos Essenciais:**

1.  **Tipos Base:**
    *   `AccountId`: Um tipo para identificar usuários (ex: `u32`).
    *   `Balance`: Um tipo para representar saldos e quantias de stake (ex: `u128`).
    *   `EraIndex`: Um tipo para identificar eras (ex: `u32`).

2.  **Estruturas de Dados e Enums:**
    *   `StakingError`: Um enum para os erros possíveis (ex: `InsufficientBalance`, `AlreadyStakedInEra`, `NotStakedInEra`, `EraNotOver`, `TargetEraInPast`, `MaxActiveStakesReached`, `StakeNotFound`, `ArithmeticOverflow`).
    *   `Event<AccountId, Balance, EraIndex>`: Um enum para representar os eventos que o pallet pode emitir (ex: `Staked { who: AccountId, amount: Balance, era: EraIndex }`, `Unstaked { who: AccountId, amount: Balance, era: EraIndex }`, `NewEra { era_index: EraIndex }`, `BalanceToppedUp { who: AccountId, amount: Balance }`).
    *   `StakerLedger<EraIndex, Balance>`: Uma struct para armazenar informações sobre o staking de um usuário.
        *   `total_staked: Balance`
   [advanced_02.md](advanced_02.md)     *   `active_staking_eras: Vec<(EraIndex, Balance)>` (uma lista de tuplas `(era, quantidade_staked_nessa_era)`)

3.  **Trait `Config`:**
    *   Defina uma trait `Config` com os seguintes tipos associados e constantes:
        *   `type AccountId: core::fmt::Debug + Eq + std::hash::Hash + Copy + Clone + Default;`
        *   `type Balance: core::fmt::Debug + Copy + Clone + Default + Ord + std::ops::AddAssign + std::ops::SubAssign + std::ops::Add<Output = Self::Balance> + std::ops::Sub<Output = Self::Balance>;`
        *   `type EraIndex: core::fmt::Debug + Copy + Clone + Default + Ord + std::ops::Add<Output = Self::EraIndex> + From<u32>;`
        *   `const MAX_ACTIVE_STAKES_PER_ACCOUNT: usize;` (número máximo de eras diferentes em que uma conta pode ter stake ativo).

4.  **Macro `ensure!`:**
    *   Implemente uma macro chamada `ensure` para verificação de condições:
        ```rust
        macro_rules! ensure {
            ($condition:expr, $error:expr) => {
                if !$condition {
                    return Err($error);
                }
            };
        }
        ```
    *   Utilize esta macro em suas funções para checagens.

5.  **Struct `Pallet<T: Config>`:**
    *   Campos para simular o storage:
        *   `balances: HashMap<T::AccountId, T::Balance>` (saldo livre de cada conta).
        *   `staker_info: HashMap<T::AccountId, StakerLedger<T::EraIndex, T::Balance>>` (informações de staking por conta).
        *   `current_era: T::EraIndex`.
        *   `events: Vec<Event<T::AccountId, T::Balance, T::EraIndex>>` (para armazenar eventos emitidos).
    *   Métodos:
        *   `new(initial_balances: Vec<(T::AccountId, T::Balance)>) -> Self`: Inicializa o pallet, populando os saldos iniciais e definindo `current_era` para 0 (ou `T::EraIndex::from(0u32)`).
        *   `stake(origin: T::AccountId, amount: T::Balance, target_era: T::EraIndex) -> Result<(), StakingError<T::Balance>>`:
            *   Permite que `origin` stake `amount` para a `target_era`.
            *   **Verificações (usando `ensure!`)**:
                *   `origin` tem saldo (`balances`) suficiente.
                *   `target_era` deve ser igual ou maior que `current_era`.
                *   `origin` não pode já ter um stake ativo na `target_era` (se já tiver, poderia ser uma função `add_stake` separada, mas para este desafio, vamos proibir).
                *   O número de `active_staking_eras` para `origin` não deve exceder `T::MAX_ACTIVE_STAKES_PER_ACCOUNT` se for uma nova era de stake.
            *   **Lógica**:
                *   Debita `amount` do saldo livre de `origin`.
                *   Atualiza `staker_info` para `origin`, adicionando ou atualizando o stake para `target_era` e o `total_staked`.
                *   Emite `Event::Staked`.
        *   `unstake(origin: T::AccountId, era_to_unstake: T::EraIndex) -> Result<(), StakingError<T::Balance>>`:
            *   Permite que `origin` resgate todo o stake da `era_to_unstake`.
            *   **Verificações (usando `ensure!`)**:
                *   `era_to_unstake` deve ser menor que `current_era` (só pode resgatar de eras passadas).
                *   `origin` deve ter um stake ativo na `era_to_unstake`.
            *   **Lógica**:
                *   Obtém a quantia stakada por `origin` na `era_to_unstake`.
                *   Credita essa quantia ao saldo livre de `origin`.
                *   Remove o stake da `era_to_unstake` de `staker_info` para `origin` e atualiza `total_staked`.
                *   Emite `Event::Unstaked`.
        *   `advance_era(&mut self) -> Result<(), StakingError<T::Balance>>`:
            *   Incrementa `current_era`.
            *   Emite `Event::NewEra`.
        *   `get_balance(&self, who: T::AccountId) -> T::Balance`: Retorna o saldo livre de `who`.
        *   `get_staked_amount(&self, who: T::AccountId, era: T::EraIndex) -> Option<T::Balance>`: Retorna a quantia que `who` stakou na `era` específica.
        *   `get_total_staked_by_account(&self, who: T::AccountId) -> T::Balance`: Retorna o total stakado por `who` em todas as eras ativas.
        *   `get_current_era(&self) -> T::EraIndex`.
        *   `drain_events(&mut self) -> Vec<Event<T::AccountId, T::Balance, T::EraIndex>>`: Retorna todos os eventos acumulados e limpa a lista interna de eventos.

6.  **Gerenciamento de Memória e Boas Práticas Rust:**
    *   Atenção ao borrowing, especialmente com `HashMap` e `&mut self`.
    *   Use `entry` API do `HashMap` onde apropriado.
    *   Use `Default::default()` ou `T::Balance::default()` para inicializar saldos/stakes zerados.

**Contexto Teórico:**

*   **Macros Declarativas (`macro_rules!`)**: Permitem definir sintaxe customizada que se expande em código Rust. `ensure!` é um padrão comum em Substrate para verificações concisas. Elas operam na árvore sintática abstrata (AST) antes da compilação completa.
    *   Referência: [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html) (Capítulos sobre `macro_rules!`)
*   **Emissão de Eventos**: Em Substrate, pallets emitem eventos para sinalizar que algo significativo ocorreu (ex: uma transferência, um novo stake). Esses eventos são armazenados no bloco e podem ser observados por clientes externos. Nossa simulação com `Vec<Event>` é uma aproximação.
*   **Tipos Associados e Trait Bounds**: A `Config` trait se torna mais poderosa com tipos associados (`type Balance = ...`). Os *trait bounds* (`Ord`, `AddAssign`, etc.) especificam quais operações devem ser possíveis com esses tipos, garantindo que a lógica do pallet funcione corretamente.
*   **StorageValue e StorageDoubleMap**:
    *   `current_era` simula um `StorageValue<EraIndex>`, um valor único armazenado.
    *   A forma como `staker_info` é usado para buscar o stake de uma `AccountId` em uma `EraIndex` específica (mesmo que aninhado dentro do `StakerLedger`) se aproxima da ideia de um `StorageDoubleMap<(AccountId, EraIndex), Balance>`, onde você precisa de duas chaves para obter um valor.

**Testes (coloque isso dentro de um módulo `tests`):**
```rust
#[cfg(test)]
mod tests {
    use super::*; // Importe as definições do seu pallet
    use std::collections::HashMap; // Para inicializar o pallet

    // Defina uma implementação mock da Config para os testes
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct MockAccountId(u32);

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct MockBalance(u128);

    impl std::ops::AddAssign for MockBalance {
        fn add_assign(&mut self, other: Self) { self.0 += other.0; }
    }
    impl std::ops::SubAssign for MockBalance {
        fn sub_assign(&mut self, other: Self) { self.0 -= other.0; }
    }
    impl std::ops::Add for MockBalance {
        type Output = Self;
        fn add(self, other: Self) -> Self { MockBalance(self.0 + other.0) }
    }
    impl std::ops::Sub for MockBalance {
        type Output = Self;
        fn sub(self, other: Self) -> Self { MockBalance(self.0 - other.0) }
    }


    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct MockEra(u32);

    impl From<u32> for MockEra {
        fn from(val: u32) -> Self { MockEra(val) }
    }
    impl std::ops::Add for MockEra {
        type Output = Self;
        fn add(self, other: Self) -> Self { MockEra(self.0 + other.0) }
    }


    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = MockAccountId;
        type Balance = MockBalance;
        type EraIndex = MockEra;
        const MAX_ACTIVE_STAKES_PER_ACCOUNT: usize = 3;
    }

    type TestPallet = Pallet<TestConfig>;
    type TestEvent = Event<MockAccountId, MockBalance, MockEra>;
    type TestStakingError = StakingError<MockBalance>;


    fn alice() -> MockAccountId { MockAccountId(1) }
    fn bob() -> MockAccountId { MockAccountId(2) }
    fn balance(val: u128) -> MockBalance { MockBalance(val) }
    fn era(val: u32) -> MockEra { MockEra(val) }

    fn setup_pallet(initial_balances_vec: Vec<(MockAccountId, MockBalance)>) -> TestPallet {
        let mut pallet = TestPallet::new(initial_balances_vec);
        pallet.drain_events(); // Clear setup events
        pallet
    }

    #[test]
    fn initial_state_and_balance() {
        let pallet = setup_pallet(vec![(alice(), balance(1000))]);
        assert_eq!(pallet.get_balance(alice()), balance(1000));
        assert_eq!(pallet.get_balance(bob()), balance(0));
        assert_eq!(pallet.get_current_era(), era(0));
    }

    #[test]
    fn stake_works_and_emits_event() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);
        
        assert_eq!(pallet.stake(alice(), balance(100), era(0)), Ok(()));
        assert_eq!(pallet.get_balance(alice()), balance(900));
        assert_eq!(pallet.get_staked_amount(alice(), era(0)), Some(balance(100)));
        assert_eq!(pallet.get_total_staked_by_account(alice()), balance(100));

        let events = pallet.drain_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], TestEvent::Staked { who: alice(), amount: balance(100), era: era(0) });
    }

    #[test]
    fn stake_fails_insufficient_balance() {
        let mut pallet = setup_pallet(vec![(alice(), balance(50))]);
        assert_eq!(
            pallet.stake(alice(), balance(100), era(0)),
            Err(TestStakingError::InsufficientBalance(balance(50)))
        );
    }

    #[test]
    fn stake_fails_target_era_in_past() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);
        pallet.advance_era().unwrap(); // current_era = 1
        pallet.drain_events();
        assert_eq!(pallet.get_current_era(), era(1));
        assert_eq!(
            pallet.stake(alice(), balance(100), era(0)),
            Err(TestStakingError::TargetEraInPast)
        );
    }
    
    #[test]
    fn stake_fails_already_staked_in_era() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);
        pallet.stake(alice(), balance(100), era(0)).unwrap();
        pallet.drain_events();
        assert_eq!(
            pallet.stake(alice(), balance(50), era(0)),
            Err(TestStakingError::AlreadyStakedInEra)
        );
    }

    #[test]
    fn stake_fails_max_active_stakes_reached() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);
        pallet.stake(alice(), balance(10), era(0)).unwrap();
        pallet.stake(alice(), balance(10), era(1)).unwrap();
        pallet.stake(alice(), balance(10), era(2)).unwrap(); // Max is 3
        pallet.drain_events();

        assert_eq!(
            pallet.stake(alice(), balance(10), era(3)),
            Err(TestStakingError::MaxActiveStakesReached(TestConfig::MAX_ACTIVE_STAKES_PER_ACCOUNT))
        );
    }
    
    #[test]
    fn advance_era_works_and_emits_event() {
        let mut pallet = setup_pallet(vec![]);
        assert_eq!(pallet.get_current_era(), era(0));
        pallet.advance_era().unwrap();
        assert_eq!(pallet.get_current_era(), era(1));
        
        let events = pallet.drain_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], TestEvent::NewEra { era_index: era(1) });
    }

    #[test]
    fn unstake_works_and_emits_event() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);
        pallet.stake(alice(), balance(100), era(0)).unwrap();
        pallet.advance_era().unwrap(); // current_era = 1, era(0) is now in the past
        pallet.drain_events();

        assert_eq!(pallet.unstake(alice(), era(0)), Ok(()));
        assert_eq!(pallet.get_balance(alice()), balance(1000)); // 900 (after stake) + 100 (unstaked)
        assert_eq!(pallet.get_staked_amount(alice(), era(0)), None);
        assert_eq!(pallet.get_total_staked_by_account(alice()), balance(0));
        
        let events = pallet.drain_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], TestEvent::Unstaked { who: alice(), amount: balance(100), era: era(0) });
    }

    #[test]
    fn unstake_fails_era_not_over() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);
        pallet.stake(alice(), balance(100), era(0)).unwrap();
        // current_era is still 0
        pallet.drain_events();
        assert_eq!(
            pallet.unstake(alice(), era(0)),
            Err(TestStakingError::EraNotOver)
        );
    }

    #[test]
    fn unstake_fails_not_staked_in_era() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);
        pallet.advance_era().unwrap(); // current_era = 1
        pallet.drain_events();
        assert_eq!(
            pallet.unstake(alice(), era(0)), // Never staked in era 0
            Err(TestStakingError::StakeNotFound)
        );
    }
    
    #[test]
    fn multiple_stakes_and_unstakes_for_account() {
        let mut pallet = setup_pallet(vec![(alice(), balance(1000))]);

        // Stake in era 0 and 1
        pallet.stake(alice(), balance(100), era(0)).unwrap();
        pallet.stake(alice(), balance(200), era(1)).unwrap();
        assert_eq!(pallet.get_balance(alice()), balance(700));
        assert_eq!(pallet.get_total_staked_by_account(alice()), balance(300));
        
        // Advance to era 1
        pallet.advance_era().unwrap(); // current_era = 1
        assert_eq!(pallet.unstake(alice(), era(0)), Ok(())); // Unstake from era 0
        assert_eq!(pallet.get_balance(alice()), balance(800)); // 700 + 100
        assert_eq!(pallet.get_total_staked_by_account(alice()), balance(200)); // Only era 1 stake remains
        assert_eq!(pallet.get_staked_amount(alice(), era(0)), None);
        assert_eq!(pallet.get_staked_amount(alice(), era(1)), Some(balance(200)));

        // Advance to era 2
        pallet.advance_era().unwrap(); // current_era = 2
        assert_eq!(pallet.unstake(alice(), era(1)), Ok(())); // Unstake from era 1
        assert_eq!(pallet.get_balance(alice()), balance(1000)); // 800 + 200
        assert_eq!(pallet.get_total_staked_by_account(alice()), balance(0));
        assert_eq!(pallet.get_staked_amount(alice(), era(1)), None);
    }
}
```

**Output Esperado:**

Seu código Rust deve compilar sem erros, e todos os testes fornecidos devem passar com sucesso. As funcionalidades do pallet de staking devem operar conforme as especificações.

---
