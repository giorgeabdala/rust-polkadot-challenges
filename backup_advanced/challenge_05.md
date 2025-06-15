
---

## Desafio 5: Ping Não Assinado com Validação

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas


### Descrição do Objetivo

Você implementará um pallet simulado que aceita uma transação não assinada chamada "ping". Esta transação, quando válida, registrará o número do bloco atual em que foi recebida. A parte central do desafio é implementar a trait `ValidateUnsigned` para proteger esta funcionalidade, principalmente limitando a frequência com que esses pings podem ser enviados para evitar spam.

**Conceitos e Estruturas a Implementar/Simular:**

1.  **`Pallet<T: Config>`:** A struct principal do nosso pallet.
    *   Armazenará `last_ping_block: Option<T::BlockNumber>` para rastrear o bloco do último ping bem-sucedido.
    *   Manterá uma lista de eventos emitidos: `emitted_events: Vec<Event>`.

2.  **`Config` Trait:**
    *   `type BlockNumber: ...` (com as operações necessárias como `Sub`, `PartialOrd`, `Copy`, `Default`).
    *   `type PingInterval: Get<Self::BlockNumber>;` (define o número mínimo de blocos que devem passar entre pings não assinados).

3.  **`Call<T: Config>` Enum:**
    *   Conterá uma variante: `PingUnsigned`.

4.  **`Event` Enum:**
    *   `PingReceived { block_number: T::BlockNumber }`

5.  **`Error` Enum:**
    *   `TooEarlyToPing` (se um ping for tentado antes do intervalo `PingInterval`).
    *   `InvalidCall` (se `validate_unsigned` for chamado com um `Call` inesperado).

6.  **`ValidateUnsigned` Trait (Simulada):**
    *   Você implementará esta trait para o seu `Pallet<T>`.
    *   Ela terá dois métodos principais: `validate_unsigned` e `pre_dispatch`.

7.  **`ValidTransaction` e `TransactionValidityError` (Simulados):**
    *   Estruturas simplificadas para representar o resultado da validação, conforme usado pela trait `ValidateUnsigned`.

### Estruturas Detalhadas a Implementar:

*   **`Config` Trait:**
    ```rust
    // Trait auxiliar para obter valores de configuração
    pub trait Get<V> {
        fn get() -> V;
    }

    pub trait Config {
        type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + core::ops::Sub<Output = Self::BlockNumber> + core::fmt::Debug;
        type PingInterval: Get<Self::BlockNumber>; // Mínimo de blocos entre pings
    }
    ```

*   **`Call<T: Config>` Enum:**
    ```rust
    // O _phantom é para usar o T genérico, simulando como seria em FRAME
    #[derive(Clone, Debug, PartialEq)]
    pub enum Call<T: Config> {
        PingUnsigned,
        _Phantom(core::marker::PhantomData<T>), // Para usar T
    }
    ```

*   **`Event<BlockNumber>` Enum:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<BlockNumber> {
        PingReceived { block_number: BlockNumber },
    }
    ```

*   **`Error` Enum:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        TooEarlyToPing,
        InvalidCall, // Se validate_unsigned for chamado com um Call que não é PingUnsigned
    }
    ```

*   **Estruturas de Validação (Simuladas):**
    ```rust
    #[derive(Debug, PartialEq, Clone)]
    pub struct ValidTransaction {
        pub priority: u64,
        pub requires: Vec<Vec<u8>>,
        pub provides: Vec<Vec<u8>>,
        pub longevity: u64, // Em número de blocos
        pub propagate: bool,
    }

    impl Default for ValidTransaction {
        fn default() -> Self {
            Self {
                priority: 0,
                requires: vec![],
                provides: vec![],
                longevity: 5, // Validade padrão de 5 blocos
                propagate: true,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum TransactionValidityError {
        Invalid(Error), // Usando nosso Error enum do pallet
        Unknown,      // Para outros tipos de erro de validade
    }

    pub type TransactionValidity = Result<ValidTransaction, TransactionValidityError>;
    ```

*   **`ValidateUnsigned<T: Config>` Trait:**
    ```rust
    pub trait ValidateUnsigned<T: Config> {
        fn validate_unsigned(
            // No FRAME, TransactionSource seria um parâmetro aqui. Omitido para simplicidade.
            call: &Call<T>,
            current_block: T::BlockNumber,
            last_ping_block: Option<T::BlockNumber>, // Passando o estado relevante
        ) -> TransactionValidity;

        fn pre_dispatch(call: &Call<T>) -> Result<(), TransactionValidityError>;
    }
    ```

*   **`Pallet<T: Config>` Struct:**
    ```rust
    pub struct Pallet<T: Config> {
        last_ping_block: Option<T::BlockNumber>,
        emitted_events: Vec<Event<T::BlockNumber>>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                last_ping_block: None,
                emitted_events: Vec::new(),
            }
        }

        // Função que seria chamada pelo "runtime" após pre_dispatch ter sucesso
        pub fn ping_unsigned_impl(&mut self, current_block: T::BlockNumber) -> Result<(), Error> {
            // A validação de tempo já foi feita em validate_unsigned.
            // Aqui, apenas executamos a lógica do dispatch.
            self.last_ping_block = Some(current_block);
            self.emitted_events.push(Event::PingReceived { block_number: current_block });
            Ok(())
        }

        pub fn take_events(&mut self) -> Vec<Event<T::BlockNumber>> {
            std::mem::take(&mut self.emitted_events)
        }

        // Método auxiliar para testes para definir o estado inicial
        #[cfg(test)]
        pub fn set_last_ping_block(&mut self, block: Option<T::BlockNumber>) {
            self.last_ping_block = block;
        }
    }
    ```

### Implementação de `ValidateUnsigned for Pallet<T>`:

Esta é a parte crucial. Você implementará esta trait para sua struct `Pallet`.

*   **`validate_unsigned(...)`:**
    1.  Verifique se o `call` é `Call::PingUnsigned`. Se não for, retorne `Err(TransactionValidityError::Invalid(Error::InvalidCall))`.
    2.  Use o `last_ping_block` (passado como parâmetro, pois a trait não tem `&self`) e `current_block` para verificar se `current_block - last_ping_block >= T::PingInterval::get()`.
        *   Se `last_ping_block` for `None`, o ping é permitido.
    3.  Se for muito cedo, retorne `Err(TransactionValidityError::Invalid(Error::TooEarlyToPing))`.
    4.  Se for válido, construa e retorne `Ok(ValidTransaction { ... })`.
        *   `provides`: `vec![b"my_pallet_ping_unsigned_tag".to_vec()]`. Isso ajuda o transaction pool a não aceitar múltiplos pings idênticos ao mesmo tempo.
        *   `longevity`: Um valor razoável, ex: `T::PingInterval::get()` (o ping é válido até que o próximo possa ser enviado).
        *   `priority`: Pode ser um valor padrão, ou talvez mais alto se pings forem importantes.
        *   `propagate`: `true`.

*   **`pre_dispatch(...)`:**
    1.  Verifique se o `call` é `Call::PingUnsigned`. Se não for (o que seria estranho se `validate_unsigned` passou), retorne um erro apropriado (ex: `Err(TransactionValidityError::Invalid(Error::InvalidCall))`).
    2.  Para este desafio, se a chamada for `PingUnsigned`, pode simplesmente retornar `Ok(())`, pois a principal lógica de validação (tempo) já foi feita em `validate_unsigned`. Em cenários mais complexos, `pre_dispatch` pode refazer algumas verificações leves ou preparar o estado.

### Testes

Crie um módulo `tests`. Você precisará de:
*   `TestBlockNumber` (ex: `u64`).
*   Struct `TestPingInterval` que implementa `Get<TestBlockNumber>`.
*   Struct `TestConfig` que implementa `crate::Config`.

**Cenários de Teste:**

*   **Validação bem-sucedida:**
    *   Sem pings anteriores: `validate_unsigned` deve retornar `Ok(ValidTransaction)`.
    *   Após intervalo suficiente: `validate_unsigned` deve retornar `Ok(ValidTransaction)`.
*   **Falha na validação:**
    *   Ping muito cedo: `validate_unsigned` deve retornar `Err(TransactionValidityError::Invalid(Error::TooEarlyToPing))`.
    *   Call inválido para `validate_unsigned`: Deve retornar `Err(TransactionValidityError::Invalid(Error::InvalidCall))`.
*   **`pre_dispatch`:**
    *   Chamada válida: `pre_dispatch` deve retornar `Ok(())`.
    *   Chamada inválida: `pre_dispatch` deve retornar erro.
*   **Execução do Ping (simulando o fluxo do runtime):**
    1.  Crie `Call::PingUnsigned{_Phantom: Default::default()}`.
    2.  Chame `Pallet::<TestConfig>::validate_unsigned(...)`. Se OK...
    3.  Chame `Pallet::<TestConfig>::pre_dispatch(...)`. Se OK...
    4.  Chame `pallet.ping_unsigned_impl(current_block)`.
    5.  Verifique se `pallet.last_ping_block` foi atualizado e se o evento `PingReceived` foi emitido.
*   Teste a sequência: ping bem-sucedido, tentativa de ping muito cedo (falha), espera do intervalo, novo ping bem-sucedido.

### Output Esperado

Uma implementação do `Pallet<T>` com a trait `ValidateUnsigned` funcional, protegendo a chamada `PingUnsigned` contra spam baseado em frequência. Todos os testes devem passar, demonstrando o fluxo correto de validação e execução.

### Contexto Teórico

*   **Transações Não Assinadas:** São transações que não se originam de uma conta específica e, portanto, não têm um remetente para pagar taxas ou ser responsabilizado diretamente da maneira usual. São úteis para dados que a própria rede precisa injetar (inerentes) ou para fontes externas confiáveis que não são contas (como oráculos, ou no nosso caso, um "ping" genérico). (Veja Chunk 203, 204 dos arquivos anexos).

*   **`ValidateUnsigned` Trait:** Essencial para transações não assinadas. Como não há taxa para impedir spam, essa trait permite que o pallet defina lógica customizada para aceitar ou rejeitar uma transação não assinada *antes* que ela entre no transaction pool ou seja executada.
    *   `validate_unsigned`: Chamado pelo transaction pool para verificar se a transação é válida. Retorna `TransactionValidity` que inclui informações como prioridade, longevidade na pool e tags (`provides`, `requires`) para gerenciar dependências e evitar duplicatas.
    *   `pre_dispatch`: Chamado pouco antes da execução da transação, após ela ter sido selecionada de um bloco. Permite uma última verificação ou preparação leve.

*   **`TransactionValidity`:** Um `Result` que, se `Ok`, contém uma struct `ValidTransaction`. Esta struct informa ao transaction pool como lidar com a transação (prioridade, por quanto tempo é válida, se deve ser propagada, etc.). Tags em `provides` são especialmente úteis; se uma transação não assinada com a mesma tag `provides` já estiver no pool, a nova geralmente será descartada.

Este desafio foca em um aspecto fundamental da segurança e robustez de um pallet que expõe funcionalidades não assinadas. 