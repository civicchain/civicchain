# CivicChain 

CivicChain é uma blockchain pública baseada em Proof of Work (PoW) que utiliza o algoritmo YesPower, otimizado para mineração em dispositivos comuns, incluindo celulares. A CivicChain suporta contratos inteligentes escritos em ink! (Rust para WebAssembly) e possui taxas de transação próximas de zero.

## Características

- **Nome da moeda**: CivicChain
- **Símbolo (ticker)**: CVX
- **Algoritmo de consenso**: Proof of Work usando YesPower-R16 
- **Recompensa por bloco**: 25 CVX inicialmente, com halving a cada 5 anos
- **Suprimento máximo total**: 29 milhões de moedas
- **Suporte a contratos inteligentes**: ink! (Rust para WebAssembly)
- **Taxas de transação**: Próximas de zero (micro-fees)

## Estrutura do Projeto

```
civicchain/
├── node/                 # Implementação do nó da blockchain
├── runtime/              # Runtime da blockchain
├── pallets/              # Pallets personalizados
│   ├── pow/              # Pallet de mineração PoW com YesPower
│   └── contracts/        # Pallet de contratos inteligentes
├── contracts/            # Exemplos de contratos inteligentes
│   └── simple_storage/   # Contrato de exemplo para armazenamento
├── cli_wallet/           # Carteira de linha de comando
├── web_wallet/           # Carteira web
└── docs/                 # Documentação
```

## Pré-requisitos

- Rust e Cargo (versão 1.70.0 ou superior)
- Git
- Node.js e npm (para a carteira web)
- Compilador C/C++ (para dependências nativas)

## Instalação Rápida

1. Clone o repositório:

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

4. Inicie o nó em modo de desenvolvimento:

```bash
./target/release/civicchain-node --dev
```

Para instruções mais detalhadas, consulte o [Manual da CivicChain](docs/manual.md).

## Carteira CLI

A carteira CLI permite interagir com a blockchain CivicChain através da linha de comando.

```bash
cd cli_wallet
cargo build --release
./target/release/civicchain-cli-wallet --help
```

## Carteira Web

A carteira web fornece uma interface gráfica para interagir com a blockchain CivicChain.

```bash
cd web_wallet
npm install
npm start
```

Acesse `http://localhost:3000` no seu navegador.

## Mineração

A CivicChain utiliza o algoritmo YesPower para Proof of Work, que é otimizado para dispositivos comuns, incluindo celulares.

### Mineração via Nó

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

## Contratos Inteligentes

A CivicChain suporta contratos inteligentes escritos em ink!, uma linguagem baseada em Rust para WebAssembly.

### Pré-requisitos para Desenvolvimento de Contratos

```bash
rustup component add rust-src --toolchain nightly
cargo install cargo-contract --version 3.0.0
```

### Compilando o Contrato de Exemplo

```bash
cd contracts/simple_storage
cargo +nightly contract build
```

Para mais informações sobre desenvolvimento de contratos, consulte a [documentação do ink!](https://paritytech.github.io/ink-docs/).

## Documentação

Para documentação detalhada, consulte:

- [Manual da CivicChain](docs/manual.md)
- [Documentação da API](docs/api.md)
- [Guia de Desenvolvimento de Contratos](docs/contracts.md)

 
## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## Contato
 - E-mail: beltranhoasimov@proton.me 
- X: [@CivicChain](https://x.com/CivicChain) 

## Agradecimentos

- [Substrate](https://substrate.io/) - Framework para desenvolvimento de blockchains
- [Parity Technologies](https://www.parity.io/) - Desenvolvedores do Substrate e ink!
- [YesPower](https://www.openwall.com/yespower/) - Algoritmo de Proof of Work
