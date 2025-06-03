# Documentação da API da CivicChain

Esta documentação descreve as APIs disponíveis para interagir com a blockchain CivicChain, incluindo RPC, WebSocket e bibliotecas de cliente.

## Sumário

1. [Visão Geral](#visão-geral)
2. [API JSON-RPC](#api-json-rpc)
3. [API WebSocket](#api-websocket)
4. [Polkadot.js API](#polkadotjs-api)
5. [API da Carteira CLI](#api-da-carteira-cli)
6. [API de Contratos](#api-de-contratos)
7. [Exemplos de Uso](#exemplos-de-uso)

## Visão Geral

A CivicChain fornece várias interfaces para interagir com a blockchain:

- **JSON-RPC**: Para chamadas síncronas e consultas
- **WebSocket**: Para assinaturas e eventos em tempo real
- **Polkadot.js API**: Biblioteca JavaScript/TypeScript para interação completa
- **Carteira CLI**: Interface de linha de comando
- **API de Contratos**: Para interagir com contratos inteligentes

## API JSON-RPC

A API JSON-RPC está disponível na porta 9933 por padrão.

### Endpoints Principais

#### Blockchain

- `chain_getBlockHash(blockNumber)`: Retorna o hash de um bloco
- `chain_getBlock(blockHash)`: Retorna os detalhes de um bloco
- `chain_getHeader(blockHash)`: Retorna o cabeçalho de um bloco
- `chain_getFinalizedHead()`: Retorna o hash do bloco finalizado mais recente

#### Estado

- `state_getStorage(key, blockHash)`: Retorna o valor de armazenamento para uma chave
- `state_getMetadata(blockHash)`: Retorna os metadados do runtime
- `state_getRuntimeVersion(blockHash)`: Retorna a versão do runtime

#### Sistema

- `system_health()`: Retorna informações sobre a saúde do nó
- `system_name()`: Retorna o nome do nó
- `system_version()`: Retorna a versão do nó
- `system_chain()`: Retorna o nome da chain

#### Autor

- `author_submitExtrinsic(extrinsic)`: Envia uma transação
- `author_pendingExtrinsics()`: Retorna as transações pendentes

#### Mineração

- `mining_getHashrate()`: Retorna a taxa de hash atual
- `mining_getMiningInfo()`: Retorna informações sobre a mineração
- `mining_submitWork(nonce, powHash, digest)`: Envia uma solução de mineração

### Exemplo de Chamada JSON-RPC

```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method":"chain_getBlockHash", "params":[1]}' http://localhost:9933
```

## API WebSocket

A API WebSocket está disponível na porta 9944 por padrão e permite assinaturas para eventos em tempo real.

### Assinaturas Principais

- `chain_subscribeNewHeads`: Assina novos cabeçalhos de blocos
- `chain_subscribeFinalizedHeads`: Assina cabeçalhos de blocos finalizados
- `state_subscribeStorage`: Assina mudanças em chaves de armazenamento
- `state_subscribeRuntimeVersion`: Assina mudanças na versão do runtime

### Exemplo de Assinatura WebSocket

```javascript
const WebSocket = require('websocket').w3cwebsocket;
const ws = new WebSocket('ws://localhost:9944');

ws.onopen = () => {
  ws.send(JSON.stringify({
    id: 1,
    jsonrpc: '2.0',
    method: 'chain_subscribeNewHeads',
    params: []
  }));
};

ws.onmessage = (message) => {
  const response = JSON.parse(message.data);
  console.log('Novo bloco:', response.params.result);
};
```

## Polkadot.js API

A biblioteca Polkadot.js API é a maneira recomendada para interagir com a CivicChain a partir de aplicativos JavaScript/TypeScript.

### Instalação

```bash
npm install @polkadot/api
```

### Conexão Básica

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function connectToChain() {
  // Conectar ao nó da CivicChain
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Obter informações da chain
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);

  console.log(`Conectado a ${chain} usando ${nodeName} v${nodeVersion}`);
  
  return api;
}
```

### Consultas Comuns

```javascript
async function commonQueries(api) {
  // Obter o bloco mais recente
  const lastHeader = await api.rpc.chain.getHeader();
  console.log('Último bloco:', lastHeader.number.toNumber());

  // Obter saldo de uma conta
  const address = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
  const { data: balance } = await api.query.system.account(address);
  console.log('Saldo:', balance.free.toString());

  // Obter informações de mineração
  const miningInfo = await api.rpc.mining.getMiningInfo();
  console.log('Informações de mineração:', miningInfo);
}
```

### Envio de Transações

```javascript
async function sendTransaction(api, account) {
  // Criar uma transação de transferência
  const txHash = await api.tx.balances
    .transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 12345)
    .signAndSend(account);

  console.log('Transação enviada com hash:', txHash.toString());
}
```

### Assinaturas

```javascript
async function subscribeToEvents(api) {
  // Assinar novos blocos
  const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
    console.log(`Novo bloco #${header.number}: ${header.hash}`);
  });

  // Para cancelar a assinatura
  // unsubscribe();
}
```

## API da Carteira CLI

A carteira CLI fornece uma interface de linha de comando para interagir com a CivicChain.

### Comandos Principais

```bash
# Gerar uma nova chave
./target/release/civicchain-cli-wallet generate-key --output=minha-chave.json

# Verificar saldo
./target/release/civicchain-cli-wallet --key-file=minha-chave.json balance

# Transferir fundos
./target/release/civicchain-cli-wallet --key-file=minha-chave.json transfer --to=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --amount=10

# Minerar
./target/release/civicchain-cli-wallet --key-file=minha-chave.json mine --threads=4

# Consultar informações de bloco
./target/release/civicchain-cli-wallet block-info --block=1234
```

### Uso Programático

A carteira CLI também pode ser usada como uma biblioteca em outros programas Rust:

```rust
use civicchain_cli_wallet::{
    commands::{balance, transfer, mine},
    config::Config,
    wallet::Wallet,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Carregar configuração
    let config = Config::from_file("config.json")?;
    
    // Carregar carteira
    let wallet = Wallet::from_file("minha-chave.json")?;
    
    // Verificar saldo
    let balance = balance::get_balance(&config, &wallet)?;
    println!("Saldo: {}", balance);
    
    // Transferir fundos
    let tx_hash = transfer::send_transfer(
        &config, 
        &wallet, 
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 
        10_000_000_000
    )?;
    println!("Transação enviada: {}", tx_hash);
    
    Ok(())
}
```

## API de Contratos

A CivicChain suporta contratos inteligentes escritos em ink!. Você pode interagir com esses contratos usando a API Polkadot.js ou a carteira CLI.

### Usando Polkadot.js API

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');

async function interactWithContract() {
  // Conectar ao nó
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Carregar ABI do contrato
  const abi = require('./path/to/metadata.json');
  
  // Endereço do contrato implantado
  const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
  
  // Criar instância do contrato
  const contract = new ContractPromise(api, abi, contractAddress);
  
  // Chamar uma função de leitura (query)
  const { result, output } = await contract.query.get(
    contractAddress,
    { gasLimit: -1 }
  );
  
  if (result.isOk) {
    console.log('Valor armazenado:', output.toString());
  }
  
  // Chamar uma função de escrita (tx)
  const account = '...'; // Sua conta
  const value = 0; // Valor a enviar com a transação
  const gasLimit = 3000000;
  
  await contract.tx
    .set({ gasLimit, value }, 42)
    .signAndSend(account, (result) => {
      console.log('Status da transação:', result.status.toString());
    });
}
```

### Usando a Carteira CLI

```bash
# Implantar contrato
./target/release/civicchain-cli-wallet --key-file=minha-chave.json deploy-contract --contract=./meu-contrato.contract --value=0 --args='{"init_value": 42}'

# Chamar método de contrato
./target/release/civicchain-cli-wallet --key-file=minha-chave.json call-contract --address=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --method=set --value=0 --args='{"new_value": 100}'
```

## Exemplos de Uso

### Monitoramento de Blocos

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function monitorBlocks() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  console.log('Monitorando novos blocos...');

  // Assinar novos blocos
  const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
    console.log(`Novo bloco #${header.number}: ${header.hash}`);
    
    // Obter detalhes do bloco
    const blockHash = await api.rpc.chain.getBlockHash(header.number);
    const block = await api.rpc.chain.getBlock(blockHash);
    
    // Contar transações
    const txCount = block.block.extrinsics.length;
    console.log(`Transações no bloco: ${txCount}`);
    
    // Verificar eventos
    const events = await api.query.system.events.at(blockHash);
    console.log(`Eventos no bloco: ${events.length}`);
  });
}

monitorBlocks().catch(console.error);
```

### Aplicativo de Mineração

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

async function startMining() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Configurar conta de mineração
  const keyring = new Keyring({ type: 'sr25519' });
  const miner = keyring.addFromUri('//Alice');

  console.log(`Iniciando mineração com a conta ${miner.address}`);

  // Função para minerar
  async function mine() {
    try {
      // Obter dados de mineração
      const miningInfo = await api.rpc.mining.getMiningInfo();
      console.log('Dificuldade atual:', miningInfo.difficulty.toString());
      
      // Simular mineração (na prática, isso seria um algoritmo de mineração real)
      console.log('Minerando...');
      
      // Enviar solução (exemplo simplificado)
      const nonce = Math.floor(Math.random() * 1000000);
      const digest = '0x...'; // Hash calculado
      
      const result = await api.rpc.mining.submitWork(nonce, miningInfo.powHash, digest);
      
      if (result) {
        console.log('Bloco minerado com sucesso!');
      } else {
        console.log('Falha ao minerar bloco, tentando novamente...');
      }
    } catch (error) {
      console.error('Erro durante mineração:', error);
    }
    
    // Continuar minerando
    setTimeout(mine, 1000);
  }

  // Iniciar mineração
  mine();
}

startMining().catch(console.error);
```

### Interação com Contrato SimpleStorage

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');
const { Keyring } = require('@polkadot/keyring');

async function simpleStorageExample() {
  // Conectar ao nó
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Configurar conta
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  // Carregar ABI do contrato
  const abi = require('./simple_storage.json');
  
  // Endereço do contrato (substitua pelo endereço real após a implantação)
  const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
  
  // Criar instância do contrato
  const contract = new ContractPromise(api, abi, contractAddress);
  
  // Ler valor atual
  const { result, output } = await contract.query.get(alice.address, { gasLimit: -1 });
  
  if (result.isOk) {
    console.log('Valor atual:', output.toString());
  }
  
  // Definir novo valor
  const gasLimit = 3000000;
  const value = 0;
  const newValue = 42;
  
  await contract.tx
    .set({ gasLimit, value }, newValue)
    .signAndSend(alice, (result) => {
      if (result.status.isInBlock) {
        console.log(`Transação incluída no bloco: ${result.status.asInBlock}`);
      } else if (result.status.isFinalized) {
        console.log(`Transação finalizada no bloco: ${result.status.asFinalized}`);
        
        // Ler valor atualizado
        contract.query.get(alice.address, { gasLimit: -1 }).then(({ output }) => {
          console.log('Novo valor:', output.toString());
        });
      }
    });
}

simpleStorageExample().catch(console.error);
```

---

Para mais informações e exemplos, consulte a [documentação completa da API](https://docs.civicchain.org/api).
