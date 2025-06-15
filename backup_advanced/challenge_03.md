
## Desafio 3: Simulando uma Migração de Storage Simples

**Nível de Dificuldade:** Avançado
**Tempo Estimado:** 2 horas

### Descrição do Objetivo

Neste desafio, você simulará uma migração de storage muito básica para um "pallet". A ideia é entender como o estado armazenado pode ser transformado quando a lógica de um pallet evolui.

Vamos considerar duas versões de um item de storage:
*   **V1:** Armazena um simples valor `u32`.
*   **V2:** Armazena uma tupla `(u32, bool)`. A intenção é que o `u32` seja o valor da V1, e o `bool` seja um novo indicador (por exemplo, `is_migrated_data: true`).

Você implementará uma estrutura que simula o storage de um pallet e uma função que realiza a migração da V1 para a V2.

**Conceitos Principais Abordados:**
*   **`Structs` e `Enums`:** Para definir a estrutura do nosso "pallet" simulado e o versionamento do storage.
*   **`Option<T>`:** Para representar valores que podem ou não existir no storage.
*   **Pattern Matching:** Para lidar com os diferentes estados e versões do storage.
*   **Lógica de Migração:** Implementar a transformação dos dados da V1 para a V2.
*   **Versionamento do Storage:** Controlar quando a migração deve ocorrer.

### Estruturas a Implementar:

1.  **`Config` Trait (Mínima):**
    ```rust
    pub trait Config {
        // Para este desafio, pode ser vazia ou definir tipos que você ache úteis,
        // mas não é estritamente necessário para a lógica principal da migração.
        // Exemplo: type Weight = u64; (para o retorno da função de migração)
    }
    ```

2.  **`StorageVersion` Enum:**
    ```rust
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum StorageVersion {
        V1_SimpleU32,
        V2_U32WithFlag,
    }
    ```

3.  **`PalletStorageSim<T: Config>` Struct:**
    *   Esta struct simula o estado do storage do nosso pallet.
    ```rust
    pub struct PalletStorageSim<T: Config> {
        // Versão atual do schema do storage.
        pub current_version: StorageVersion,

        // Simula o storage da V1. Contém o valor se a versão for V1_SimpleU32.
        // Será 'None' após a migração bem-sucedida para V2.
        storage_v1_value: Option<u32>,

        // Simula o storage da V2. Contém o valor se a versão for V2_U32WithFlag.
        // Será preenchido durante a migração.
        storage_v2_value: Option<(u32, bool)>,

        _phantom: core::marker::PhantomData<T>, // Para usar o T: Config
    }
    ```

### Métodos do `PalletStorageSim<T: Config>`:

*   `pub fn new() -> Self`
    *   Inicializa `current_version` para `StorageVersion::V1_SimpleU32`.
    *   Inicializa `storage_v1_value` e `storage_v2_value` para `None`.
*   `pub fn set_initial_v1_value(&mut self, value: u32)`
    *   Define `storage_v1_value` com `Some(value)`.
    *   **Importante:** Esta função só deve ter efeito se `current_version` for `V1_SimpleU32`. Se já estiver em V2, pode-se optar por não fazer nada ou retornar um erro/panic (para este desafio, não fazer nada é suficiente).
*   `pub fn get_current_v2_value(&self) -> Option<(u32, bool)>`
    *   Retorna uma cópia de `storage_v2_value` **somente se** `current_version` for `V2_U32WithFlag`. Caso contrário, retorna `None`.
*   `pub fn run_migration_if_needed(&mut self) -> u64 /* Peso simulado */`
    *   Esta função simula o hook `on_runtime_upgrade` que seria chamado durante um upgrade de runtime.
    *   Verifica `self.current_version`:
        *   Se for `StorageVersion::V1_SimpleU32`:
            *   Realiza a migração:
                *   Se `self.storage_v1_value` for `Some(old_val)`, então `self.storage_v2_value` se torna `Some((old_val, true))`.
                *   Se `self.storage_v1_value` for `None`, então `self.storage_v2_value` se torna `None`.
            *   "Limpa" o storage antigo: `self.storage_v1_value = None`.
            *   Atualiza a versão: `self.current_version = StorageVersion::V2_U32WithFlag`.
            *   Retorna um "peso" simulado (ex: `2` para indicar 1 leitura e 2 escritas - versão e valor novo). Se o valor V1 era `None`, o peso pode ser `1` (1 leitura da versão, 1 escrita da versão).
        *   Se já for `StorageVersion::V2_U32WithFlag` (ou qualquer versão mais recente, se houvesse):
            *   Nenhuma ação é necessária.
            *   Retorna um peso `0`.

### Testes

Crie um módulo `tests` e use uma struct `TestConfig` simples.

**Cenários de Teste:**

1.  **Inicialização:**
    *   Verificar se `PalletStorageSim::new()` define `current_version` para `V1_SimpleU32` e os valores para `None`.
2.  **Definir Valor V1:**
    *   Criar pallet, chamar `set_initial_v1_value(100)`.
    *   Verificar se `storage_v1_value` é `Some(100)`.
    *   Verificar se `get_current_v2_value()` retorna `None`.
3.  **Migração com Valor Existente:**
    *   Definir um valor V1 (ex: `100`).
    *   Chamar `run_migration_if_needed()`.
    *   Verificar se `current_version` é `V2_U32WithFlag`.
    *   Verificar se `storage_v1_value` é `None`.
    *   Verificar se `storage_v2_value` é `Some((100, true))`.
    *   Verificar se `get_current_v2_value()` retorna `Some((100, true))`.
    *   Verificar se o peso retornado é > 0.
4.  **Migração com Valor V1 Ausente:**
    *   Criar pallet (sem chamar `set_initial_v1_value`).
    *   Chamar `run_migration_if_needed()`.
    *   Verificar se `current_version` é `V2_U32WithFlag`.
    *   Verificar se `storage_v1_value` é `None`.
    *   Verificar se `storage_v2_value` é `None`.
    *   Verificar se `get_current_v2_value()` retorna `None`.
    *   Verificar se o peso retornado é > 0 (pela atualização da versão).
5.  **Tentativa de Migração Dupla:**
    *   Realizar uma migração bem-sucedida.
    *   Chamar `run_migration_if_needed()` novamente.
    *   Verificar se o estado (`current_version`, `storage_v1_value`, `storage_v2_value`) permanece inalterado.
    *   Verificar se o peso retornado é `0`.
6.  **Tentativa de Definir Valor V1 Após Migração:**
    *   Realizar uma migração.
    *   Tentar chamar `set_initial_v1_value(200)`.
    *   Verificar se `storage_v1_value` permanece `None` (ou o comportamento que você definiu para este caso) e `storage_v2_value` não é afetado.

### Output Esperado

Uma implementação funcional do `PalletStorageSim<T>` e seus métodos, passando em todos os testes unitários. O código deve demonstrar claramente a lógica da migração e o controle de versão.

### Contexto Teórico

*   **Runtime Upgrades:** No Substrate, a lógica da blockchain (o runtime, compilado para Wasm) pode ser atualizada on-chain. Isso permite que a blockchain evolua sem hard forks. (Referência: Chunks 67, 68 dos arquivos anexos).
*   **Storage Migrations:** Quando a estrutura dos dados armazenados por um pallet muda em uma nova versão do runtime, uma migração de storage é necessária. Essa migração é um código que roda uma única vez durante o upgrade para transformar os dados do formato antigo para o novo. (Referência: Chunks 70, 71).
*   **`OnRuntimeUpgrade` Trait:** Pallets podem implementar esta trait. Sua função `on_runtime_upgrade()` é chamada pelo `Executive` pallet durante o processo de upgrade do runtime, após o novo código do runtime ser implantado, mas antes que qualquer outra coisa (como `on_initialize` ou transações) seja processada. (Referência: Chunks 68, 70, 71, 74).
*   **`StorageVersion`:** É uma prática comum que pallets mantenham sua própria versão de "schema" de storage. A lógica de migração verifica essa versão para decidir se a migração deve ser executada. (Referência: Chunk 72, 73).
    *   `VersionedMigration` é um helper comum para isso no FRAME, mas estamos simulando o conceito manualmente.
*   **Importância:** Sem migrações corretas, a nova versão do runtime poderia falhar ao tentar ler dados antigos ou interpretá-los incorretamente, levando a inconsistências ou panics.

Este desafio simplificado foca na mecânica central da transformação de dados e no versionamento, que são cruciais para entender as migrações de storage no Substrate.
