# Guia de Desenvolvimento de Contratos Inteligentes na CivicChain

Este guia fornece instruções detalhadas para desenvolver, testar, implantar e interagir com contratos inteligentes na blockchain CivicChain usando a linguagem ink!.

## Sumário

1. [Introdução ao ink!](#introdução-ao-ink)
2. [Configuração do Ambiente](#configuração-do-ambiente)
3. [Criando seu Primeiro Contrato](#criando-seu-primeiro-contrato)
4. [Estrutura de um Contrato ink!](#estrutura-de-um-contrato-ink)
5. [Compilação e Teste](#compilação-e-teste)
6. [Implantação de Contratos](#implantação-de-contratos)
7. [Interação com Contratos](#interação-com-contratos)
8. [Padrões e Melhores Práticas](#padrões-e-melhores-práticas)
9. [Segurança de Contratos](#segurança-de-contratos)
10. [Exemplos Avançados](#exemplos-avançados)
11. [Solução de Problemas](#solução-de-problemas)

## Introdução ao ink!

ink! é uma linguagem de programação baseada em Rust para escrever contratos inteligentes para a máquina virtual WebAssembly (Wasm). A CivicChain utiliza ink! como sua linguagem principal para contratos inteligentes.

### Vantagens do ink!

- **Segurança**: Herda as garantias de segurança de memória do Rust
- **Desempenho**: Compilado para WebAssembly, oferecendo execução rápida e eficiente
- **Interoperabilidade**: Compatível com o ecossistema Substrate/Polkadot
- **Ferramentas Modernas**: Suporte a testes, documentação e análise estática
- **Tipagem Forte**: Detecção de erros em tempo de compilação

### Comparação com Outras Linguagens de Contrato

| Característica | ink! | Solidity (Ethereum) | CosmWasm (Cosmos) |
|----------------|------|---------------------|-------------------|
| Linguagem Base | Rust | Própria             | Rust              |
| Compilação     | Wasm | EVM Bytecode        | Wasm              |
| Tipagem        | Forte | Forte              | Forte             |
| Maturidade     | Média | Alta               | Média             |
| Segurança      | Alta  | Média              | Alta              |
| Ecossistema    | Em crescimento | Extenso   | Em crescimento    |

## Configuração do Ambiente

### Pré-requisitos

- Rust e Cargo (versão 1.70.0 ou superior)
- Toolchain nightly do Rust
- cargo-contract (ferramenta CLI para ink!)

### Instalação

1. Instale o Rust e Cargo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Instale a toolchain nightly e o target WebAssembly:

```bash
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

3. Instale a ferramenta cargo-contract:

```bash
cargo install cargo-contract --version 3.0.0
```

4. Verifique a instalação:

```bash
cargo contract --version
```

## Criando seu Primeiro Contrato

### Inicialização de um Novo Projeto

```bash
cargo contract new simple_storage
cd simple_storage
```

### Estrutura do Projeto

```
simple_storage/
├── .gitignore
├── Cargo.toml
├── lib.rs           # Código-fonte do contrato
└── .cargo/
    └── config.toml  # Configuração do cargo
```

### Contrato SimpleStorage Básico

Abra o arquivo `lib.rs` e substitua seu conteúdo pelo seguinte:

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_storage {
    #[ink(storage)]
    pub struct SimpleStorage {
        value: u32,
    }

    impl SimpleStorage {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: 0 }
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn set(&mut self, new_value: u32) {
            self.value = new_value;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let simple_storage = SimpleStorage::default();
            assert_eq!(simple_storage.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut simple_storage = SimpleStorage::new(42);
            assert_eq!(simple_storage.get(), 42);
            simple_storage.set(100);
            assert_eq!(simple_storage.get(), 100);
        }
    }
}
```

## Estrutura de um Contrato ink!

### Atributos Principais

- `#[ink::contract]`: Marca um módulo como um contrato ink!
- `#[ink(storage)]`: Define a estrutura de armazenamento do contrato
- `#[ink(constructor)]`: Marca uma função como construtor (chamada na implantação)
- `#[ink(message)]`: Marca uma função como mensagem (método público)
- `#[ink(event)]`: Define um evento que pode ser emitido pelo contrato

### Tipos de Armazenamento

- `Mapping<K, V>`: Mapeamento de chave-valor (similar a um HashMap)
- `Vec<T>`: Vetor dinâmico
- `Option<T>`: Valor opcional
- `AccountId`: Endereço de uma conta
- `Balance`: Tipo para representar saldos
- `Hash`: Tipo para representar hashes

### Exemplo de Uso de Mapping

```rust
#[ink(storage)]
pub struct MyContract {
    balances: ink::storage::Mapping<AccountId, Balance>,
}

impl MyContract {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut balances = ink::storage::Mapping::default();
        let caller = Self::env().caller();
        balances.insert(caller, &1000);
        Self { balances }
    }

    #[ink(message)]
    pub fn get_balance(&self, account: AccountId) -> Balance {
        self.balances.get(account).unwrap_or(0)
    }
}
```

### Eventos

```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: AccountId,
    value: Balance,
}

// Emitir um evento
self.env().emit_event(Transfer {
    from: Some(from),
    to,
    value,
});
```

## Compilação e Teste

### Compilação de Contrato

```bash
# Na pasta do projeto
cargo +nightly contract build
```

Isso gerará os seguintes arquivos na pasta `target/ink`:

- `your_contract.contract`: Pacote completo (código Wasm + metadados)
- `your_contract.wasm`: Código WebAssembly compilado
- `metadata.json`: Metadados do contrato (ABI)

### Execução de Testes

```bash
# Testes unitários
cargo +nightly test

# Testes específicos
cargo +nightly test it_works
```

### Testes com Ambiente de Contrato

```rust
#[ink::test]
fn test_transfer() {
    // Configurar contas para teste
    let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
    
    // Configurar o chamador do contrato
    let contract = ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
    
    // Criar o contrato
    let mut contract = MyContract::new();
    
    // Executar a função a ser testada
    contract.transfer(accounts.bob, 100);
    
    // Verificar o resultado
    assert_eq!(contract.get_balance(accounts.alice), 900);
    assert_eq!(contract.get_balance(accounts.bob), 100);
}
```

## Implantação de Contratos

### Usando a Carteira Web

1. Acesse a carteira web da CivicChain em `http://localhost:3000`
2. Navegue até a seção "Contratos"
3. Clique em "Implantar Contrato"
4. Selecione o arquivo `.contract` gerado
5. Preencha os argumentos do construtor
6. Clique em "Implantar"

### Usando a Carteira CLI

```bash
./target/release/civicchain-cli-wallet --key-file=minha-chave.json deploy-contract --contract=./target/ink/simple_storage.contract --value=0 --args='{"init_value": 42}'
```

### Usando Polkadot.js API

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');
const fs = require('fs');

async function deployContract() {
  // Conectar ao nó
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Carregar o contrato
  const contractFile = fs.readFileSync('./target/ink/simple_storage.contract');
  const contract = JSON.parse(contractFile);
  
  // Configurar conta
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  
  // Implantar o contrato
  const gasLimit = 3000000;
  const value = 0;
  
  // Criar instância do contrato
  const tx = api.tx.contracts.instantiateWithCode(
    value,
    gasLimit,
    contract.wasm,
    contract.abi.constructors[0].args,
    0x00
  );
  
  // Enviar transação
  const result = await tx.signAndSend(alice);
  console.log('Contrato implantado:', result.toString());
}

deployContract().catch(console.error);
```

## Interação com Contratos

### Usando a Carteira Web

1. Acesse a carteira web da CivicChain
2. Navegue até a seção "Contratos"
3. Insira o endereço do contrato
4. Selecione o método a chamar
5. Preencha os argumentos
6. Clique em "Chamar"

### Usando a Carteira CLI

```bash
# Chamar método de leitura (get)
./target/release/civicchain-cli-wallet --key-file=minha-chave.json call-contract --address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --method=get

# Chamar método de escrita (set)
./target/release/civicchain-cli-wallet --key-file=minha-chave.json call-contract --address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --method=set --args='{"new_value": 100}'
```

### Usando Polkadot.js API

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');

async function interactWithContract() {
  // Conectar ao nó
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Carregar ABI do contrato
  const abi = require('./target/ink/metadata.json');
  
  // Endereço do contrato implantado
  const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
  
  // Criar instância do contrato
  const contract = new ContractPromise(api, abi, contractAddress);
  
  // Chamar método de leitura (get)
  const { result, output } = await contract.query.get(
    '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
    { gasLimit: -1 }
  );
  
  if (result.isOk) {
    console.log('Valor atual:', output.toString());
  }
  
  // Chamar método de escrita (set)
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  
  await contract.tx
    .set({ gasLimit: 3000000, value: 0 }, 42)
    .signAndSend(alice, (result) => {
      console.log('Status da transação:', result.status.toString());
    });
}

interactWithContract().catch(console.error);
```

## Padrões e Melhores Práticas

### Padrão de Propriedade (Ownable)

```rust
#[ink(storage)]
pub struct Ownable {
    owner: AccountId,
    // outros campos...
}

impl Ownable {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self {
            owner: Self::env().caller(),
            // inicialização de outros campos...
        }
    }

    #[ink(message)]
    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        assert_eq!(self.env().caller(), self.owner, "Apenas o proprietário pode transferir a propriedade");
        self.owner = new_owner;
    }

    fn only_owner(&self) {
        assert_eq!(self.env().caller(), self.owner, "Apenas o proprietário pode chamar esta função");
    }
}
```

### Padrão de Pausa (Pausable)

```rust
#[ink(storage)]
pub struct Pausable {
    paused: bool,
    owner: AccountId,
    // outros campos...
}

impl Pausable {
    #[ink(message)]
    pub fn pause(&mut self) {
        assert_eq!(self.env().caller(), self.owner, "Apenas o proprietário pode pausar");
        self.paused = true;
    }

    #[ink(message)]
    pub fn unpause(&mut self) {
        assert_eq!(self.env().caller(), self.owner, "Apenas o proprietário pode despausar");
        self.paused = false;
    }

    fn when_not_paused(&self) {
        assert!(!self.paused, "Contrato está pausado");
    }
}
```

### Padrão de Controle de Acesso por Função (Role-Based Access Control)

```rust
#[ink(storage)]
pub struct AccessControl {
    roles: ink::storage::Mapping<(AccountId, [u8; 32]), bool>,
    // outros campos...
}

impl AccessControl {
    #[ink(message)]
    pub fn grant_role(&mut self, role: [u8; 32], account: AccountId) {
        // Verificar permissão
        self.roles.insert((account, role), &true);
    }

    #[ink(message)]
    pub fn revoke_role(&mut self, role: [u8; 32], account: AccountId) {
        // Verificar permissão
        self.roles.remove((account, role));
    }

    fn has_role(&self, role: [u8; 32], account: AccountId) -> bool {
        self.roles.get((account, role)).unwrap_or(false)
    }
}
```

## Segurança de Contratos

### Vulnerabilidades Comuns e Mitigações

| Vulnerabilidade | Descrição | Mitigação |
|-----------------|-----------|-----------|
| Reentrância | Um contrato chama outro contrato que pode chamar de volta o primeiro contrato | Padrão checks-effects-interactions |
| Overflow/Underflow | Operações aritméticas que excedem os limites do tipo | Usar tipos seguros ou verificar limites |
| Acesso não autorizado | Funções sensíveis acessíveis por qualquer usuário | Implementar controle de acesso |
| Manipulação de timestamp | Dependência de timestamps que podem ser manipulados | Evitar dependência crítica de timestamps |
| Negação de serviço | Loops infinitos ou operações muito caras | Limitar loops e verificar custos de gás |

### Exemplo de Proteção contra Reentrância

```rust
#[ink(message)]
pub fn withdraw(&mut self, amount: Balance) {
    let caller = self.env().caller();
    let caller_balance = self.balances.get(caller).unwrap_or(0);
    
    // Verificações
    assert!(caller_balance >= amount, "Saldo insuficiente");
    
    // Efeitos (atualizar estado)
    self.balances.insert(caller, &(caller_balance - amount));
    
    // Interações (transferências externas)
    if self.env().transfer(caller, amount).is_err() {
        // Reverter as alterações de estado em caso de falha
        self.balances.insert(caller, &caller_balance);
        panic!("Falha na transferência");
    }
}
```

### Verificações de Segurança

- **Auditoria de Código**: Revisão manual do código por especialistas
- **Testes Automatizados**: Cobertura completa de testes unitários e de integração
- **Análise Estática**: Ferramentas como clippy para detectar problemas comuns
- **Fuzzing**: Testes com entradas aleatórias para encontrar casos extremos
- **Implantação Gradual**: Começar com testnet, depois mainnet com valores limitados

## Exemplos Avançados

### Contrato de Token ERC-20

```rust
#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &initial_supply);
            
            Self::env().emit_event(Transfer {
                from: None,
                to: caller,
                value: initial_supply,
            });
            
            Self {
                total_supply: initial_supply,
                balances,
                allowances: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            
            true
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or(0)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            
            if allowance < value {
                return false;
            }
            
            if !self.transfer_from_to(from, to, value) {
                return false;
            }
            
            self.allowances.insert((from, caller), &(allowance - value));
            true
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return false;
            }
            
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            
            self.env().emit_event(Transfer {
                from: Some(from),
                to,
                value,
            });
            
            true
        }
    }
}
```

### Contrato Multisig

```rust
#[ink::contract]
mod multisig {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Multisig {
        owners: Vec<AccountId>,
        threshold: u32,
        transaction_count: u32,
        transactions: Mapping<u32, Transaction>,
        confirmations: Mapping<(u32, AccountId), bool>,
    }

    #[derive(scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Transaction {
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
        executed: bool,
    }

    #[ink(event)]
    pub struct Submission {
        #[ink(topic)]
        tx_id: u32,
    }

    #[ink(event)]
    pub struct Confirmation {
        #[ink(topic)]
        sender: AccountId,
        #[ink(topic)]
        tx_id: u32,
    }

    #[ink(event)]
    pub struct Execution {
        #[ink(topic)]
        tx_id: u32,
    }

    impl Multisig {
        #[ink(constructor)]
        pub fn new(owners: Vec<AccountId>, threshold: u32) -> Self {
            assert!(owners.len() >= threshold as usize, "Threshold too high");
            assert!(threshold > 0, "Threshold must be positive");
            
            Self {
                owners,
                threshold,
                transaction_count: 0,
                transactions: Mapping::default(),
                confirmations: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn submit_transaction(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> u32 {
            let caller = self.env().caller();
            assert!(self.is_owner(caller), "Not an owner");
            
            let tx_id = self.transaction_count;
            self.transactions.insert(tx_id, &Transaction {
                to,
                value,
                data,
                executed: false,
            });
            
            self.transaction_count += 1;
            self.env().emit_event(Submission { tx_id });
            
            self.confirm_transaction(tx_id);
            tx_id
        }

        #[ink(message)]
        pub fn confirm_transaction(&mut self, tx_id: u32) -> bool {
            let caller = self.env().caller();
            assert!(self.is_owner(caller), "Not an owner");
            assert!(self.transaction_exists(tx_id), "Transaction does not exist");
            assert!(!self.is_confirmed(tx_id, caller), "Transaction already confirmed");
            
            self.confirmations.insert((tx_id, caller), &true);
            self.env().emit_event(Confirmation { sender: caller, tx_id });
            
            if self.is_confirmed_by_threshold(tx_id) {
                self.execute_transaction(tx_id);
                return true;
            }
            
            false
        }

        #[ink(message)]
        pub fn execute_transaction(&mut self, tx_id: u32) -> bool {
            assert!(self.transaction_exists(tx_id), "Transaction does not exist");
            assert!(!self.is_executed(tx_id), "Transaction already executed");
            assert!(self.is_confirmed_by_threshold(tx_id), "Not enough confirmations");
            
            let transaction = self.transactions.get(tx_id).unwrap();
            
            // Executar a transação
            let result = self.env().transfer(transaction.to, transaction.value);
            
            if result.is_ok() {
                let mut tx = transaction.clone();
                tx.executed = true;
                self.transactions.insert(tx_id, &tx);
                
                self.env().emit_event(Execution { tx_id });
                return true;
            }
            
            false
        }

        #[ink(message)]
        pub fn get_confirmation_count(&self, tx_id: u32) -> u32 {
            let mut count = 0;
            for owner in &self.owners {
                if self.is_confirmed(tx_id, *owner) {
                    count += 1;
                }
            }
            count
        }

        fn is_owner(&self, account: AccountId) -> bool {
            self.owners.contains(&account)
        }

        fn transaction_exists(&self, tx_id: u32) -> bool {
            self.transactions.get(tx_id).is_some()
        }

        fn is_confirmed(&self, tx_id: u32, owner: AccountId) -> bool {
            self.confirmations.get((tx_id, owner)).unwrap_or(false)
        }

        fn is_executed(&self, tx_id: u32) -> bool {
            if let Some(tx) = self.transactions.get(tx_id) {
                tx.executed
            } else {
                false
            }
        }

        fn is_confirmed_by_threshold(&self, tx_id: u32) -> bool {
            self.get_confirmation_count(tx_id) >= self.threshold
        }
    }
}
```

## Solução de Problemas

### Problemas Comuns e Soluções

#### Erro: "Could not find `Cargo.toml`"

**Solução**: Certifique-se de estar no diretório correto do projeto.

#### Erro: "Error: Failed to compile the contract"

**Solução**: Verifique erros de sintaxe ou dependências ausentes no Cargo.toml.

#### Erro: "Error: Failed to instantiate the contract"

**Solução**: Verifique se os argumentos do construtor estão corretos e se há saldo suficiente para pagar as taxas de gás.

#### Erro: "Error: Failed to call the contract"

**Solução**: Verifique se o endereço do contrato está correto e se os argumentos da função estão no formato correto.

#### Erro: "Error: Out of gas"

**Solução**: Aumente o limite de gás para a transação ou otimize o contrato para usar menos gás.

### Depuração de Contratos

- Use `ink_env::debug_println!` para imprimir mensagens de depuração durante os testes
- Adicione eventos para rastrear o fluxo de execução
- Use testes unitários para verificar partes específicas do contrato
- Implante em uma testnet antes da mainnet

### Recursos Adicionais

- [Documentação oficial do ink!](https://paritytech.github.io/ink-docs/)
- [Exemplos de contratos ink!](https://github.com/paritytech/ink/tree/master/examples)
- [Fórum da CivicChain](https://forum.civicchain.org)
- [Canal Discord da CivicChain](https://discord.gg/civicchain)

---

Este guia fornece uma introdução ao desenvolvimento de contratos inteligentes na CivicChain usando ink!. Para informações mais detalhadas, consulte a documentação oficial ou entre em contato com a comunidade.
