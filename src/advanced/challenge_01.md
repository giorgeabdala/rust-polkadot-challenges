## Desafio 1: Pallet Simplificado de Ativos Únicos (NFTs)

**Nível de Dificuldade:** Avançado

**Descrição do Objetivo:**

Você deve implementar a lógica central de um "pallet" simplificado para gerenciamento de ativos únicos (semelhantes a NFTs). Este pallet permitirá a criação e transferência de ativos, garantindo que cada ativo seja único e tenha um proprietário.

**Requisitos Essenciais:**

1.  **Estruturas de Dados:**
    *   `AssetId`: Um tipo para identificar unicamente cada ativo (pode ser um `u32`).
    *   `AccountId`: Um tipo para identificar proprietários (pode ser um `u32`).
    *   `AssetDetails`: Uma struct contendo:
        *   `owner: AccountId`
        *   `metadata: Vec<u8>` (um vetor de bytes para metadados simples)
    *   `PalletError`: Um enum para representar os possíveis erros (ex: `AssetAlreadyExists`, `AssetNotFound`, `NotOwner`, `ArithmeticOverflow`).

2.  **Simulação de Configuração e Storage:**
    *   Crie uma trait `Config` que defina um tipo associado `MaxMetadataLength` (um `u32`) para limitar o tamanho dos metadados.
    *   Implemente uma struct `Pallet<T: Config>` que atuará como nosso pallet.
    *   Dentro de `Pallet<T: Config>`, use `std::collections::HashMap` para simular o armazenamento:
        *   `assets: HashMap<AssetId, AssetDetails>`: Para armazenar os detalhes de cada ativo.
        *   `owner_assets: HashMap<AccountId, Vec<AssetId>>`: Para rastrear quais ativos pertencem a cada conta (opcional, mas bom para certas consultas).

3.  **Funcionalidades (Métodos da `Pallet<T: Config>`):**
    *   `new() -> Self`: Construtor para inicializar o pallet (e seus storages vazios).
    *   `mint(origin: AccountId, asset_id: AssetId, metadata: Vec<u8>) -> Result<(), PalletError>`:
        *   Cria um novo ativo com o `asset_id` fornecido e o atribui ao `origin`.
        *   Verifica se o `asset_id` já existe. Se sim, retorna `PalletError::AssetAlreadyExists`.
        *   Verifica se o tamanho de `metadata` excede `T::MaxMetadataLength`. Se sim, retorna `PalletError::MetadataTooLong` (adicione este erro ao enum).
        *   Adiciona o ativo ao storage `assets`.
        *   (Opcional) Atualiza `owner_assets`.
    *   `transfer(origin: AccountId, to: AccountId, asset_id: AssetId) -> Result<(), PalletError>`:
        *   Transfere a propriedade do `asset_id` de `origin` para `to`.
        *   Verifica se o `asset_id` existe. Se não, retorna `PalletError::AssetNotFound`.
        *   Verifica se `origin` é o proprietário atual do ativo. Se não, retorna `PalletError::NotOwner`.
        *   Atualiza o proprietário do ativo em `assets`.
        *   (Opcional) Atualiza `owner_assets` para ambos, `origin` e `to`.
    *   `get_asset_owner(asset_id: AssetId) -> Option<AccountId>`:
        *   Retorna o proprietário do `asset_id`, ou `None` se o ativo não existir.
    *   `get_asset_metadata(asset_id: AssetId) -> Option<Vec<u8>>`:
        *   Retorna os metadados do `asset_id`, ou `None` se o ativo não existir.

4.  **Gerenciamento de Memória e Boas Práticas Rust:**
    *   Preste atenção ao ownership e borrowing, especialmente ao interagir com o `HashMap`.
    *   Use `Result` e `Option` de forma idiomática.
    *   Implemente `Debug` para suas structs e enums para facilitar os testes.

**Contexto Teórico:**

*   **Pallets em Substrate:** Pallets são módulos que encapsulam a lógica de negócios de uma blockchain construída com Substrate. Eles definem tipos de dados, armazenamento, funções dispatchable (extrínsecos), eventos e erros. Nosso desafio simplifica essa estrutura. Em um pallet real, você usaria macros como `#[pallet::config]`, `#[pallet::storage]`, `#[pallet::event]`, `#[pallet::error]`, `#[pallet::call]`. Aqui, simularemos a `Config` trait e o armazenamento com `HashMap`.
*   **`Config` Trait:** Em Substrate, a trait `Config` é usada para parametrizar um pallet, permitindo que ele acesse tipos e constantes definidos pelo runtime que o utiliza. Isso promove a modularidade e a reutilização.
*   **Storage em Substrate:** O Substrate fornece tipos de armazenamento otimizados (`StorageMap`, `StorageValue`, `StorageDoubleMap`, `StorageNMap`) que são mapeados para uma árvore de Merkle Patricia subjacente. `HashMap` é uma aproximação razoável para este exercício em memória.
*   **`Option<T>` e `Result<T, E>`:** São fundamentais em Rust para tratamento de valores que podem estar ausentes e para operações que podem falhar, respectivamente. O pattern matching (`match`, `if let`, `while let`) é a forma mais comum de trabalhar com eles.
    *   Referência `Option`: [https://doc.rust-lang.org/std/option/enum.Option.html](https://doc.rust-lang.org/std/option/enum.Option.html)
    *   Referência `Result`: [https://doc.rust-lang.org/std/result/enum.Result.html](https://doc.rust-lang.org/std/result/enum.Result.html)

**Testes (coloque isso dentro de um módulo `tests`):**

```rust
#[cfg(test)]
mod tests {
    use super::*; // Importe as definições do seu pallet

    // Defina uma implementação mock da Config para os testes
    struct TestConfig;
    impl Config for TestConfig {
        const MAX_METADATA_LENGTH: u32 = 64; // Exemplo de limite
    }

    type TestPallet = Pallet<TestConfig>;

    #[test]
    fn initial_state() {
        let pallet = TestPallet::new();
        assert_eq!(pallet.get_asset_owner(0), None);
    }

    #[test]
    fn mint_asset_works() {
        let mut pallet = TestPallet::new();
        let owner_alice: AccountId = 1;
        let asset_id_nft: AssetId = 100;
        let metadata = vec![1, 2, 3];

        assert_eq!(pallet.mint(owner_alice, asset_id_nft, metadata.clone()), Ok(()));
        assert_eq!(pallet.get_asset_owner(asset_id_nft), Some(owner_alice));
        assert_eq!(pallet.get_asset_metadata(asset_id_nft), Some(metadata));
    }

    #[test]
    fn mint_asset_fails_if_already_exists() {
        let mut pallet = TestPallet::new();
        let owner_alice: AccountId = 1;
        let asset_id_nft: AssetId = 100;
        let metadata = vec![1, 2, 3];

        pallet.mint(owner_alice, asset_id_nft, metadata.clone()).unwrap();
        assert_eq!(
            pallet.mint(owner_alice, asset_id_nft, metadata),
            Err(PalletError::AssetAlreadyExists)
        );
    }

    #[test]
    fn mint_asset_fails_if_metadata_too_long() {
        let mut pallet = TestPallet::new();
        let owner_alice: AccountId = 1;
        let asset_id_nft: AssetId = 101;
        let long_metadata = vec![0u8; (TestConfig::MAX_METADATA_LENGTH + 1) as usize];

        assert_eq!(
            pallet.mint(owner_alice, asset_id_nft, long_metadata),
            Err(PalletError::MetadataTooLong)
        );
    }

    #[test]
    fn transfer_asset_works() {
        let mut pallet = TestPallet::new();
        let owner_alice: AccountId = 1;
        let owner_bob: AccountId = 2;
        let asset_id_nft: AssetId = 100;
        let metadata = vec![1, 2, 3];

        pallet.mint(owner_alice, asset_id_nft, metadata).unwrap();
        assert_eq!(pallet.transfer(owner_alice, owner_bob, asset_id_nft), Ok(()));
        assert_eq!(pallet.get_asset_owner(asset_id_nft), Some(owner_bob));
    }

    #[test]
    fn transfer_asset_fails_if_not_owner() {
        let mut pallet = TestPallet::new();
        let owner_alice: AccountId = 1;
        let owner_bob: AccountId = 2;
        let hacker_charlie: AccountId = 3;
        let asset_id_nft: AssetId = 100;
        let metadata = vec![1, 2, 3];

        pallet.mint(owner_alice, asset_id_nft, metadata).unwrap();
        assert_eq!(
            pallet.transfer(hacker_charlie, owner_bob, asset_id_nft),
            Err(PalletError::NotOwner)
        );
    }

    #[test]
    fn transfer_asset_fails_if_asset_not_found() {
        let mut pallet = TestPallet::new();
        let owner_alice: AccountId = 1;
        let owner_bob: AccountId = 2;
        let non_existent_asset_id: AssetId = 999;

        assert_eq!(
            pallet.transfer(owner_alice, owner_bob, non_existent_asset_id),
            Err(PalletError::AssetNotFound)
        );
    }
    
    // (Opcional) Testes para owner_assets, se implementado
    #[test]
    fn owner_assets_tracking_works_on_mint_and_transfer() {
        let mut pallet = TestPallet::new();
        let alice: AccountId = 1;
        let bob: AccountId = 2;
        let asset1: AssetId = 101;
        let asset2: AssetId = 102;

        // Mint
        pallet.mint(alice, asset1, vec![1]).unwrap();
        pallet.mint(alice, asset2, vec![2]).unwrap();
        
        // Verifique se owner_assets_of existe e se os ativos estão lá
        // Esta é uma sugestão, você precisará de uma função como `get_assets_by_owner`
        // ou tornar owner_assets público para teste (menos ideal)
        // Por agora, vamos assumir que você adicionará uma função auxiliar ou testará indiretamente.
        // Se você adicionou owner_assets, este teste é um bom lugar para verificá-lo.
        // Exemplo: assert_eq!(pallet.get_assets_by_owner(alice), Some(vec![asset1, asset2]));

        // Transfer
        pallet.transfer(alice, bob, asset1).unwrap();
        // Exemplo: assert_eq!(pallet.get_assets_by_owner(alice), Some(vec![asset2]));
        // Exemplo: assert_eq!(pallet.get_assets_by_owner(bob), Some(vec![asset1]));
        
        // Para este teste passar como está, sem uma função `get_assets_by_owner`,
        // você pode apenas garantir que o código compila e, na sua análise,
        // pode verificar manualmente se a lógica de owner_assets foi implementada.
        // Para uma verificação real, uma função getter seria necessária.
        assert!(true, "Implementar verificações para owner_assets se a funcionalidade foi adicionada");
    }
}
```

**Output Esperado:**

Seu código deve compilar sem erros e todos os testes fornecidos devem passar. As funções devem se comportar conforme especificado nos requisitos.

---
