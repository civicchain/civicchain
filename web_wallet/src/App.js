import React, { useState, useEffect } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable, web3FromSource } from '@polkadot/extension-dapp';
import { Keyring } from '@polkadot/keyring';
import { stringToU8a, u8aToHex } from '@polkadot/util';
import { ContractPromise } from '@polkadot/api-contract';
import { BN } from 'bn.js';

import './App.css';

// Componente principal da aplicação
function App() {
  const [api, setApi] = useState(null);
  const [accounts, setAccounts] = useState([]);
  const [selectedAccount, setSelectedAccount] = useState(null);
  const [balance, setBalance] = useState(null);
  const [blockNumber, setBlockNumber] = useState(0);
  const [blockHash, setBlockHash] = useState('');
  const [transferAddress, setTransferAddress] = useState('');
  const [transferAmount, setTransferAmount] = useState('');
  const [contractAddress, setContractAddress] = useState('');
  const [contractValue, setContractValue] = useState('');
  const [contractMethod, setContractMethod] = useState('');
  const [contractArgs, setContractArgs] = useState('');
  const [mining, setMining] = useState(false);
  const [status, setStatus] = useState('Conectando à blockchain...');
  const [error, setError] = useState('');

  // Conectar à blockchain ao carregar a página
  useEffect(() => {
    const connectToBlockchain = async () => {
      try {
        // Conectar ao nó local da CivicChain
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        setApi(api);
        setStatus('Conectado à blockchain CivicChain');

        // Configurar assinatura para novos blocos
        const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
          setBlockNumber(header.number.toNumber());
          setBlockHash(header.hash.toString());
        });

        // Habilitar extensão de carteira
        const extensions = await web3Enable('CivicChain Web Wallet');
        if (extensions.length === 0) {
          setStatus('Por favor, instale a extensão Polkadot.js e autorize esta aplicação');
          return;
        }

        // Obter contas da extensão
        const accounts = await web3Accounts();
        setAccounts(accounts);
        if (accounts.length > 0) {
          setSelectedAccount(accounts[0]);
          updateBalance(api, accounts[0].address);
        }

        return () => unsubscribe();
      } catch (error) {
        console.error('Erro ao conectar à blockchain:', error);
        setStatus('Falha ao conectar à blockchain');
        setError(error.message);
      }
    };

    connectToBlockchain();
  }, []);

  // Atualizar saldo da conta selecionada
  const updateBalance = async (api, address) => {
    try {
      const { data: balance } = await api.query.system.account(address);
      setBalance(balance.free.toString());
    } catch (error) {
      console.error('Erro ao obter saldo:', error);
      setError(error.message);
    }
  };

  // Alterar conta selecionada
  const handleAccountChange = (event) => {
    const selectedAddress = event.target.value;
    const account = accounts.find(acc => acc.address === selectedAddress);
    setSelectedAccount(account);
    if (api && account) {
      updateBalance(api, account.address);
    }
  };

  // Enviar transação de transferência
  const handleTransfer = async () => {
    if (!api || !selectedAccount) {
      setError('API ou conta não disponível');
      return;
    }

    try {
      setStatus('Enviando transferência...');
      
      // Obter injetor para a conta selecionada
      const injector = await web3FromSource(selectedAccount.meta.source);
      
      // Criar e enviar a transação
      const txHash = await api.tx.balances
        .transfer(transferAddress, new BN(transferAmount).mul(new BN(10).pow(new BN(18))))
        .signAndSend(selectedAccount.address, { signer: injector.signer });
      
      setStatus(`Transferência enviada com hash: ${txHash.toString()}`);
      
      // Atualizar saldo após a transferência
      setTimeout(() => {
        updateBalance(api, selectedAccount.address);
      }, 5000);
    } catch (error) {
      console.error('Erro ao transferir:', error);
      setStatus('Falha na transferência');
      setError(error.message);
    }
  };

  // Iniciar mineração
  const handleStartMining = async () => {
    if (!api || !selectedAccount) {
      setError('API ou conta não disponível');
      return;
    }

    try {
      setMining(true);
      setStatus('Iniciando mineração...');
      
      // Implementar lógica de mineração
      // ...
      
      setStatus('Mineração em andamento...');
    } catch (error) {
      console.error('Erro ao iniciar mineração:', error);
      setStatus('Falha ao iniciar mineração');
      setError(error.message);
      setMining(false);
    }
  };

  // Parar mineração
  const handleStopMining = () => {
    setMining(false);
    setStatus('Mineração interrompida');
  };

  // Chamar método de contrato
  const handleCallContract = async () => {
    if (!api || !selectedAccount || !contractAddress) {
      setError('API, conta ou endereço do contrato não disponível');
      return;
    }

    try {
      setStatus('Chamando contrato...');
      
      // Implementar chamada de contrato
      // ...
      
      setStatus('Chamada de contrato enviada');
    } catch (error) {
      console.error('Erro ao chamar contrato:', error);
      setStatus('Falha na chamada de contrato');
      setError(error.message);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>CivicChain Web Wallet</h1>
        <div className="blockchain-status">
          <p>Status: {status}</p>
          <p>Bloco atual: {blockNumber} (Hash: {blockHash.substring(0, 10)}...)</p>
          {error && <p className="error">Erro: {error}</p>}
        </div>
      </header>

      <main>
        <section className="account-section">
          <h2>Conta</h2>
          <div className="account-selector">
            <label>Selecione uma conta:</label>
            <select onChange={handleAccountChange} value={selectedAccount?.address || ''}>
              {accounts.map(account => (
                <option key={account.address} value={account.address}>
                  {account.meta.name} ({account.address.substring(0, 10)}...)
                </option>
              ))}
            </select>
          </div>
          
          {selectedAccount && (
            <div className="account-info">
              <p>Endereço: {selectedAccount.address}</p>
              <p>Saldo: {balance ? `${new BN(balance).div(new BN(10).pow(new BN(18))).toString()} CVX` : 'Carregando...'}</p>
            </div>
          )}
        </section>

        <section className="transfer-section">
          <h2>Transferir CVX</h2>
          <div className="form-group">
            <label>Endereço do destinatário:</label>
            <input 
              type="text" 
              value={transferAddress} 
              onChange={(e) => setTransferAddress(e.target.value)} 
              placeholder="5..." 
            />
          </div>
          <div className="form-group">
            <label>Quantidade (CVX):</label>
            <input 
              type="number" 
              value={transferAmount} 
              onChange={(e) => setTransferAmount(e.target.value)} 
              placeholder="0.0" 
              min="0" 
              step="0.1" 
            />
          </div>
          <button onClick={handleTransfer} disabled={!api || !selectedAccount}>Transferir</button>
        </section>

        <section className="mining-section">
          <h2>Mineração</h2>
          {!mining ? (
            <button onClick={handleStartMining} disabled={!api || !selectedAccount}>Iniciar Mineração</button>
          ) : (
            <button onClick={handleStopMining}>Parar Mineração</button>
          )}
        </section>

        <section className="contract-section">
          <h2>Contratos Inteligentes</h2>
          <div className="form-group">
            <label>Endereço do contrato:</label>
            <input 
              type="text" 
              value={contractAddress} 
              onChange={(e) => setContractAddress(e.target.value)} 
              placeholder="5..." 
            />
          </div>
          <div className="form-group">
            <label>Método:</label>
            <input 
              type="text" 
              value={contractMethod} 
              onChange={(e) => setContractMethod(e.target.value)} 
              placeholder="get" 
            />
          </div>
          <div className="form-group">
            <label>Argumentos (JSON):</label>
            <input 
              type="text" 
              value={contractArgs} 
              onChange={(e) => setContractArgs(e.target.value)} 
              placeholder="{}" 
            />
          </div>
          <div className="form-group">
            <label>Valor (CVX):</label>
            <input 
              type="number" 
              value={contractValue} 
              onChange={(e) => setContractValue(e.target.value)} 
              placeholder="0.0" 
              min="0" 
              step="0.1" 
            />
          </div>
          <button onClick={handleCallContract} disabled={!api || !selectedAccount || !contractAddress}>Chamar Contrato</button>
        </section>
      </main>

      <footer>
        <p>CivicChain Web Wallet &copy; 2025</p>
      </footer>
    </div>
  );
}

export default App;
