
## Desafio 4: Teletransporte de Ativos Simples via XCM Mockado

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você simulará uma transferência de um ativo fungível entre duas "chains" mockadas, Chain A e Chain B. Cada chain terá um "pallet de ativos" simples para gerenciar os saldos de seus usuários para um tipo específico de ativo (vamos chamá-lo de `SimulatedAsset`).

A Chain A construirá uma mensagem XCM simplificada para instruir a Chain B a creditar ativos a um beneficiário. O foco será na definição das estruturas de mensagem XCM, na lógica de debitar ativos na origem e creditar na destinação após o "recebimento" e processamento da mensagem.

**Conceitos Principais Abordados (Simulados):**

1.  **Estruturas de Mensagem XCM Simplificadas:**
    *   `SimpleLocation`: Para identificar uma chain ou uma conta dentro de uma chain.
    *   `SimpleAsset`: Para representar o ativo e a quantidade a ser transferida.
    *   `SimpleXcmInstruction`: Comandos básicos como `WithdrawAssetFromSender` (implícito na origem) e `DepositAssetToBeneficiary`.
    *   `SimpleXcmMessage`: Uma lista de instruções.
2.  **`Structs` e `Enums`:** Para definir os tipos acima, `Event`s e `Error`s.
3.  **`Traits` e `Generics`:**
    *   `ChainConfig`: Uma trait para configurar cada chain com `AccountId`, `AssetId`, `Balance` e um identificador único para a própria chain (`ChainId`).
4.  **`std::collections::HashMap`:** Para simular o `StorageMap` dos saldos de ativos em cada chain.
5.  **Lógica de Negócio:**
    *   Debitar ativos do remetente na chain de origem.
    *   Processar a mensagem XCM na chain de destino para creditar o beneficiário.
    *   Validações básicas (saldo suficiente, chain de destino válida).
6.  **`Option<T>` e `Result<T, E>`:** Para tratamento de erros e valores.
7.  **`Pattern Matching`:** Para processar as instruções XCM e origens.

### Estruturas Detalhadas a Implementar:

*   **`ChainId`:**
    ```rust
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ChainId(pub u32);
    ```

*   **`SimpleLocation<AccountId>`:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub enum SimpleLocation<AccountId> {
        ThisChain, // Refere-se à chain atual
        SiblingChainAccount { chain_id: ChainId, account: AccountId }, // Conta em outra chain
    }
    ```
    *Nota: Para este desafio, focaremos em `DepositAssetToBeneficiary` onde o beneficiário é uma `AccountId` na chain de destino. A `SimpleLocation` será mais usada para especificar o destino da mensagem em si e o beneficiário.*

*   **`SimulatedAssetId` e `Balance`:**
    ```rust
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum SimulatedAssetId {
        MainToken, // Nosso único ativo simulado
    }
    pub type Balance = u128;
    ```

*   **`SimpleAsset`:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub struct SimpleAsset {
        pub id: SimulatedAssetId,
        pub amount: Balance,
    }
    ```

*   **`SimpleXcmInstruction<AccountId>`:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub enum SimpleXcmInstruction<AccountId> {
        // Instrução para a chain de destino
        DepositAssetToBeneficiary {
            asset: SimpleAsset,
            beneficiary: AccountId, // A conta na chain de destino
        },
        // Poderíamos ter WithdrawAsset, mas na origem isso será uma ação implícita
        // de debitar do remetente antes de enviar a XCM.
    }
    ```

*   **`SimpleXcmMessage<AccountId>`:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub struct SimpleXcmMessage<AccountId> {
        pub instructions: Vec<SimpleXcmInstruction<AccountId>>,
    }
    ```

*   **`ChainConfig` Trait:**
    ```rust
    pub trait ChainConfig {
        type AccountId: Clone + PartialEq + core::fmt::Debug + Eq + core::hash::Hash;
        type AssetId: Clone + Copy + PartialEq + core::fmt::Debug + Eq + core::hash::Hash;
        type Balance: Clone + Copy + PartialEq + core::fmt::Debug + PartialOrd + std::ops::AddAssign + std::ops::SubAssign;

        fn this_chain_id() -> ChainId;
    }
    ```

*   **`Event<C: ChainConfig>` Enum:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<C: ChainConfig> {
        AssetTeleportInitiated {
            from_account: C::AccountId,
            to_chain: ChainId,
            to_account: C::AccountId,
            asset_id: C::AssetId,
            amount: C::Balance,
        },
        AssetDepositedViaXcm {
            from_chain_hint: Option<ChainId>, // De onde a XCM pode ter vindo (opcional)
            to_account: C::AccountId,
            asset_id: C::AssetId,
            amount: C::Balance,
        },
    }
    ```

*   **`Error` Enum:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        InsufficientBalance,
        InvalidDestinationChain, // Se tentar enviar para si mesma ou chain inválida
        UnsupportedAsset,
        CannotSendZeroAmount,
        // Erros no processamento da XCM pela chain destino
        XcmProcessingError(String), // Erro genérico para o processamento da XCM
    }
    ```

*   **`AssetPallet<C: ChainConfig>` Struct (para cada chain):**
    ```rust
    pub struct AssetPallet<C: ChainConfig> {
        // Mapeia (AccountId, AssetId) -> Balance
        balances: std::collections::HashMap<(C::AccountId, C::AssetId), C::Balance>,
        emitted_events: Vec<Event<C>>,
        // phantom: std::marker::PhantomData<C>, // Se não usar C em campos, mas apenas em tipos associados e métodos
    }
    ```

### Métodos do `AssetPallet<C: ChainConfig>`:

*   `pub fn new() -> Self`
*   `fn deposit_event(&mut self, event: Event<C>)`
*   `pub fn take_events(&mut self) -> Vec<Event<C>>`
*   `pub fn balance_of(&self, account: &C::AccountId, asset_id: &C::AssetId) -> C::Balance` (retorna 0 se não houver entrada)
*   `pub fn set_balance(&mut self, account: C::AccountId, asset_id: C::AssetId, amount: C::Balance)` (helper para testes)

*   **`pub fn initiate_teleport_asset (`**
    *   `&mut self,`
    *   `sender: C::AccountId,`
    *   `destination_chain_id: ChainId,`
    *   `beneficiary_on_destination: C::AccountId,`
    *   `asset_id_to_send: C::AssetId,`
    *   `amount_to_send: C::Balance`
    *   `) -> Result<SimpleXcmMessage<C::AccountId>, Error>`
        *   Verifica se `destination_chain_id` não é `C::this_chain_id()`. Se for, `Error::InvalidDestinationChain`.
        *   Verifica se `amount_to_send` > 0. Se não, `Error::CannotSendZeroAmount`.
        *   Verifica se `asset_id_to_send` é o `SimulatedAssetId::MainToken` (ou o ativo suportado). Se não, `Error::UnsupportedAsset`.
        *   Verifica o saldo do `sender` para `asset_id_to_send`. Se insuficiente, `Error::InsufficientBalance`.
        *   Debita `amount_to_send` do `sender`.
        *   Constrói uma `SimpleXcmMessage` com uma instrução `DepositAssetToBeneficiary { asset: SimpleAsset { id: asset_id_to_send (convertido para SimulatedAssetId), amount: amount_to_send }, beneficiary: beneficiary_on_destination }`.
        *   Emite evento `Event::AssetTeleportInitiated`.
        *   Retorna `Ok(xcm_message)` que seria "enviada" para a `destination_chain_id`.

*   **`pub fn process_incoming_xcm_message (`**
    *   `&mut self,`
    *   `source_chain_hint: Option<ChainId>,`
    *   `message: SimpleXcmMessage<C::AccountId>`
    *   `) -> Result<(), Error>`
        *   Itera sobre `message.instructions`.
        *   Para cada `SimpleXcmInstruction::DepositAssetToBeneficiary`:
            *   Verifica se `asset.id` é `SimulatedAssetId::MainToken`. Se não, `Error::XcmProcessingError("Unsupported asset in XCM".into())`.
            *   Credita `asset.amount` ao `beneficiary`. (Some ao saldo existente ou crie uma nova entrada).
            *   Emite evento `Event::AssetDepositedViaXcm`.
        *   Se alguma instrução não for suportada ou falhar, retorne um `Error::XcmProcessingError`.
        *   Retorna `Ok(())` se todas as instruções forem processadas com sucesso.

### Testes

Você precisará de duas instâncias de `AssetPallet`, uma para Chain A e outra para Chain B, cada uma com sua própria `ChainConfig` (especialmente `this_chain_id()` diferente).

*   **Configuração de Teste:**
    *   `TestAccountId` (ex: `String` ou `u32`)
    *   `TestChainAConfig` e `TestChainBConfig` implementando `ChainConfig`.
        ```rust
        // Exemplo
        struct TestChainAConfig;
        impl ChainConfig for TestChainAConfig {
            type AccountId = String;
            type AssetId = SimulatedAssetId;
            type Balance = u128;
            fn this_chain_id() -> ChainId { ChainId(1) }
        }
        // Similar para TestChainBConfig com ChainId(2)
        ```
*   **Cenários de Teste:**
    *   **Teletransporte com Sucesso:**
        *   Chain A: Usuário Alice tem 100 `MainToken`.
        *   Alice (Chain A) inicia teletransporte de 30 `MainToken` para Bob na Chain B.
        *   Verificar:
            *   Saldo de Alice na Chain A é 70.
            *   Evento `AssetTeleportInitiated` emitido na Chain A.
            *   A função `initiate_teleport_asset` retorna uma `SimpleXcmMessage` correta.
        *   Chain B: Processa a `SimpleXcmMessage` recebida.
        *   Verificar:
            *   Saldo de Bob na Chain B é 30 (ou +30 se já tinha saldo).
            *   Evento `AssetDepositedViaXcm` emitido na Chain B.
    *   **Erros em `initiate_teleport_asset`:**
        *   Saldo insuficiente.
        *   Enviar para a própria chain.
        *   Enviar quantidade zero.
    *   **Erros em `process_incoming_xcm_message`:**
        *   Mensagem XCM com ativo não suportado.

### Output Esperado

Uma implementação funcional dos `AssetPallet`s e das estruturas XCM simplificadas, permitindo a simulação da transferência de ativos entre duas "chains". Todos os testes devem passar, demonstrando a lógica correta de débito, crédito e tratamento de mensagens.

### Contexto Teórico

*   **XCM (Cross-Consensus Messaging):** Como mencionado, XCM é um formato, não um protocolo de transporte. Ele define o que uma mensagem *significa*, não como ela chega lá. Nossa simulação foca no significado (as instruções `DepositAssetToBeneficiary`) e abstrai completamente o transporte.
    *   Chunks relevantes no seu arquivo: 100 (Introdução ao pallet-xcm), 234 (XCM como espinha dorsal da comunicação), 90 (Princípios do XCM), 91 (Exemplo XCM).
*   **`pallet-xcm`:** O pallet real que facilita o envio, execução e gerenciamento de mensagens XCM. Ele lida com coisas como versionamento, barreiras de segurança, execução de instruções XCM, etc.
*   **`MultiLocation`:** Em XCM real, `MultiLocation` é uma estrutura muito mais complexa para identificar locais de forma precisa através de múltiplas redes e consensos (ex: `parents: u8`, `interior: Junctions`). Nossa `SimpleLocation` é uma grande simplificação.
*   **`MultiAsset`:** Similarmente, `MultiAsset` pode representar ativos fungíveis, não fungíveis, e coleções de ativos, com identificadores que podem ser abstratos ou concretos. `SimpleAsset` foca apenas em um ID e quantidade fungível.
*   **Instruções XCM:** XCM define um vasto conjunto de instruções (veja o repositório XCM da Parity para a lista completa). `WithdrawAsset` e `DepositAsset` são fundamentais para transferências. `BuyExecution` é usado para pagar taxas pela execução da XCM. Nosso desafio simplifica para focar na lógica de depósito.

Este desafio deve dar uma boa intuição sobre como as mensagens XCM podem ser usadas para orquestrar ações entre diferentes sistemas de consenso (nossas "chains" simuladas). 