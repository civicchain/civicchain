# Manual da CivicChain

**Versão 1.0.0**

## Sumário

1. [Introdução](#introdução)
2. [Instalação](#instalação)
3. [Configuração](#configuração)
4. [Execução Local](#execução-local)
5. [Mineração](#mineração)
6. [Carteira CLI](#carteira-cli)
7. [Carteira Web](#carteira-web)
8. [Contratos Inteligentes](#contratos-inteligentes)
9. [Implantação em Mainnet/Testnet](#implantação-em-mainnet-testnet)
10. [Solução de Problemas](#solução-de-problemas)
11. [Referências](#referências)

## Introdução

A CivicChain é uma blockchain pública baseada em Proof of Work (PoW) que utiliza o algoritmo YesPower, otimizado para mineração em dispositivos comuns, incluindo celulares. A CivicChain suporta contratos inteligentes escritos em ink! (Rust para WebAssembly) e possui taxas de transação próximas de zero.

### Características Principais

- **Nome da moeda**: CivicChain
- **Símbolo (ticker)**: CVX
- **Algoritmo de consenso**: Proof of Work usando YesPower
- **Recompensa por bloco**: 60 CVX inicialmente, com halving a cada 5 anos
- **Suprimento máximo total**: 29 milhões de moedas
- **Suporte a contratos inteligentes**: ink! (Rust para WebAssembly)
- **Taxas de transação**: Próximas de zero (micro-fees)

## Instalação

### Pré-requisitos

- Rust e Cargo (versão 1.70.0 ou superior)
- Git
- Node.js e npm (para a carteira web)
- Compilador C/C++ (para dependências nativas)

### Passos de Instalação

1. Clone o repositório da CivicChain:

```bash
git clone https://github.com/civicchain/civicchain.git
cd civicchain
```

2. Instale as dependências do Rust:

```bash
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

3. Compile o nó da CivicChain:

```bash
cargo build --release
```

4. Instale a carteira CLI:

```bash
cd cli_wallet
cargo build --release
```

5. Instale a carteira web:

```bash
cd ../web_wallet
npm install
```

## Configuração

### Configuração do Nó

O nó da CivicChain pode ser configurado através de um arquivo de configuração ou parâmetros de linha de comando. O arquivo de configuração padrão é `config.json` e deve ser colocado no diretório raiz do nó.

Exemplo de configuração:

```json
{
  "chain": "dev",
  "name": "meu-no",
  "base-path": "./data",
  "port": 30333,
  "ws-port": 9944,
  "rpc-port": 9933,
  "telemetry-url": "",
  "validator": true
}
```

### Configuração da Carteira CLI

A carteira CLI não requer configuração adicional, mas você pode criar um arquivo de configuração para armazenar suas preferências:

```json
{
  "node-url": "ws://127.0.0.1:9944",
  "account-file": "./my-account.json"
}
```

### Configuração da Carteira Web

A carteira web pode ser configurada editando o arquivo `.env` no diretório `web_wallet`:

```
REACT_APP_NODE_URL=ws://127.0.0.1:9944
REACT_APP_DEFAULT_NETWORK=local
```

## Execução Local

### Iniciar o Nó em Modo de Desenvolvimento

```bash
./target/release/civicchain-node --dev
```

### Iniciar o Nó em Modo de Testnet Local

```bash
./target/release/civicchain-node --chain=local
```

### Iniciar o Nó com Configurações Personalizadas

```bash
./target/release/civicchain-node --chain=local --base-path=./my-chain-data --port=30334 --ws-port=9945 --rpc-port=9934
```

### Iniciar a Carteira Web

```bash
cd web_wallet
npm start
```

A carteira web estará disponível em `http://localhost:3000`.

## Mineração

A CivicChain utiliza o algoritmo YesPower para Proof of Work, que é otimizado para dispositivos comuns, incluindo celulares.

### Mineração via Nó

Você pode minerar diretamente através do nó da CivicChain:

```bash
./target/release/civicchain-node --mine --miner-account=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

### Mineração via Carteira CLI

```bash
./target/release/civicchain-cli-wallet --dev-account=alice mine --threads=4
```

### Mineração via Carteira Web

1. Acesse a carteira web em `http://localhost:3000`
2. Conecte-se à sua conta
3. Navegue até a seção "Mineração"
4. Clique em "Iniciar Mineração"

### Mineração em Dispositivos Móveis

Para minerar em dispositivos móveis, você pode usar o aplicativo CivicChain Mobile Miner (em desenvolvimento). Alternativamente, você pode acessar a carteira web através do navegador do seu dispositivo móvel e iniciar a mineração a partir dela.

## Carteira CLI

A carteira CLI permite interagir com a blockchain CivicChain através da linha de comando.

### Comandos Básicos

- **Gerar uma nova chave**:

```bash
./target/release/civicchain-cli-wallet generate-key --output=minha-chave.json
```

- **Verificar saldo**:

```bash
./target/release/civicchain-cli-wallet --key-file=minha-chave.json balance
```

- **Transferir fundos**:

```bash
./target/release/civicchain-cli-wallet --key-file=minha-chave.json transfer --to=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --amount=10
```

- **Minerar**:

```bash
./target/release/civicchain-cli-wallet --key-file=minha-chave.json mine --threads=4
```

- **Consultar informações de bloco**:

```bash
./target/release/civicchain-cli-wallet block-info --block=1234
```

### Interação com Contratos

- **Implantar contrato**:

```bash
./target/release/civicchain-cli-wallet --key-file=minha-chave.json deploy-contract --contract=./meu-contrato.contract --value=0 --args='{"init_value": 42}'
```

- **Chamar método de contrato**:

```bash
./target/release/civicchain-cli-wallet --key-file=minha-chave.json call-contract --address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --method=set --value=0 --args='{"new_value": 100}'
```

## Carteira Web

A carteira web fornece uma interface gráfica para interagir com a blockchain CivicChain.

### Funcionalidades

- Gerenciamento de contas
- Transferência de fundos
- Consulta de saldo
- Mineração
- Implantação e interação com contratos inteligentes
- Consulta de informações da blockchain

### Uso da Carteira Web

1. Inicie a carteira web:

```bash
cd web_wallet
npm start
```

2. Acesse `http://localhost:3000` no seu navegador
3. Conecte-se à sua conta usando a extensão Polkadot.js
4. Navegue pelas diferentes seções para interagir com a blockchain

## Contratos Inteligentes

A CivicChain suporta contratos inteligentes escritos em ink!, uma linguagem baseada em Rust para WebAssembly.

### Pré-requisitos para Desenvolvimento de Contratos

```bash
rustup component add rust-src --toolchain nightly
cargo install cargo-contract --version 3.0.0
```

### Criando um Novo Contrato

```bash
cargo contract new meu-contrato
cd meu-contrato
```

### Compilando um Contrato

```bash
cargo +nightly contract build
```

### Testando um Contrato

```bash
cargo +nightly test
```

### Implantando um Contrato

Você pode implantar um contrato usando a carteira CLI ou a carteira web.

#### Via Carteira CLI:

```bash
./target/release/civicchain-cli-wallet --key-file=minha-chave.json deploy-contract --contract=./target/ink/meu-contrato.contract --value=0 --args='{}'
```

#### Via Carteira Web:

1. Acesse a carteira web
2. Navegue até a seção "Contratos"
3. Clique em "Implantar Contrato"
4. Selecione o arquivo `.contract`
5. Preencha os argumentos do construtor
6. Clique em "Implantar"

### Exemplo de Contrato SimpleStorage

O contrato SimpleStorage é um exemplo simples que armazena e recupera um valor numérico:

```rust
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

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn set(&mut self, new_value: u32) {
            self.value = new_value;
        }
    }
}
```

## Implantação em Mainnet/Testnet

### Implantação em Testnet

Para implantar um nó na testnet da CivicChain:

```bash
./target/release/civicchain-node --chain=testnet --name=meu-no-testnet --port=30333 --ws-port=9944 --rpc-port=9933
```

### Implantação em Mainnet

Para implantar um nó na mainnet da CivicChain:

```bash
./target/release/civicchain-node --chain=mainnet --name=meu-no-mainnet --port=30333 --ws-port=9944 --rpc-port=9933
```

### Considerações para Implantação em Produção

- Use um servidor dedicado com recursos adequados (CPU, RAM, armazenamento)
- Configure um firewall para permitir apenas as portas necessárias
- Use um sistema de monitoramento para acompanhar o desempenho do nó
- Configure backups regulares dos dados da blockchain
- Considere usar um balanceador de carga se estiver executando múltiplos nós

## Solução de Problemas

### Problemas Comuns e Soluções

#### O nó não sincroniza

- Verifique sua conexão com a internet
- Verifique se as portas necessárias estão abertas no firewall
- Tente conectar-se a outros nós manualmente usando o parâmetro `--bootnodes`

#### Erro ao compilar o nó

- Verifique se você tem a versão correta do Rust e as ferramentas necessárias
- Limpe a compilação e tente novamente: `cargo clean && cargo build --release`

#### Erro ao minerar

- Verifique se a conta que você está usando tem saldo suficiente para pagar as taxas de transação
- Verifique se o nó está sincronizado com a rede

#### Erro ao implantar contratos

- Verifique se o contrato foi compilado corretamente
- Verifique se você tem saldo suficiente para pagar as taxas de implantação
- Verifique se os argumentos do construtor estão corretos

### Logs e Depuração

Para obter logs mais detalhados, inicie o nó com o parâmetro `--log=debug`:

```bash
./target/release/civicchain-node --dev --log=debug
```

## Referências

- [Documentação do Substrate](https://docs.substrate.io/)
- [Documentação do ink!](https://paritytech.github.io/ink-docs/)
- [Especificação do YesPower](https://www.openwall.com/yespower/)
- [Polkadot.js API](https://polkadot.js.org/docs/api/)
- [Repositório da CivicChain](https://github.com/civicchain/civicchain)

---

© 2025 CivicChain. Todos os direitos reservados.
