//! # CivicChain CLI Wallet
//!
//! Uma carteira de linha de comando para interagir com a blockchain CivicChain.
//! Permite enviar transações, minerar blocos, consultar o estado da blockchain,
//! implantar e chamar contratos inteligentes.

use clap::{Parser, Subcommand};
use sp_core::{crypto::Ss58Codec, sr25519, Pair, H256, U256};
use sp_keyring::AccountKeyring;
use std::{fs, path::PathBuf};
use substrate_api_client::{
    compose_extrinsic, rpc::WsRpcClient, Api, UncheckedExtrinsicV4, XtStatus,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// URL do nó CivicChain
    #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
    url: String,

    /// Caminho para o arquivo da chave
    #[arg(short, long)]
    key_file: Option<PathBuf>,

    /// Usar conta de desenvolvimento (alice, bob, charlie, etc.)
    #[arg(short, long)]
    dev_account: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Consultar o saldo de uma conta
    Balance {
        /// Endereço da conta (formato SS58)
        #[arg(short, long)]
        address: Option<String>,
    },

    /// Transferir CVX para outra conta
    Transfer {
        /// Endereço do destinatário (formato SS58)
        #[arg(short, long)]
        to: String,

        /// Quantidade de CVX a transferir
        #[arg(short, long)]
        amount: u128,
    },

    /// Minerar blocos
    Mine {
        /// Número de threads para mineração
        #[arg(short, long, default_value = "1")]
        threads: u8,
    },

    /// Consultar informações sobre um bloco
    BlockInfo {
        /// Hash do bloco ou número
        #[arg(short, long)]
        block: Option<String>,
    },

    /// Implantar um contrato inteligente
    DeployContract {
        /// Caminho para o arquivo .contract
        #[arg(short, long)]
        contract: PathBuf,

        /// Valor inicial a enviar para o contrato
        #[arg(short, long, default_value = "0")]
        value: u128,

        /// Argumentos do construtor (formato JSON)
        #[arg(short, long, default_value = "{}")]
        args: String,
    },

    /// Chamar um método de contrato
    CallContract {
        /// Endereço do contrato
        #[arg(short, long)]
        address: String,

        /// Nome do método a chamar
        #[arg(short, long)]
        method: String,

        /// Valor a enviar com a chamada
        #[arg(short, long, default_value = "0")]
        value: u128,

        /// Argumentos do método (formato JSON)
        #[arg(short, long, default_value = "{}")]
        args: String,
    },

    /// Gerar um novo par de chaves
    GenerateKey {
        /// Nome do arquivo de saída
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Conectar ao nó
    println!("Conectando ao nó CivicChain em {}...", cli.url);
    let client = WsRpcClient::new(&cli.url)?;
    
    // Obter o par de chaves
    let pair = if let Some(dev_account) = cli.dev_account {
        match dev_account.to_lowercase().as_str() {
            "alice" => AccountKeyring::Alice.pair(),
            "bob" => AccountKeyring::Bob.pair(),
            "charlie" => AccountKeyring::Charlie.pair(),
            "dave" => AccountKeyring::Dave.pair(),
            "eve" => AccountKeyring::Eve.pair(),
            "ferdie" => AccountKeyring::Ferdie.pair(),
            _ => return Err("Conta de desenvolvimento inválida".into()),
        }
    } else if let Some(key_file) = cli.key_file {
        let seed = fs::read_to_string(key_file)?;
        sr25519::Pair::from_string(&seed, None).map_err(|_| "Falha ao carregar a chave")?
    } else {
        return Err("Nenhuma chave fornecida. Use --key-file ou --dev-account".into());
    };

    let api = Api::<sr25519::Pair, _>::new(client).map_err(|e| format!("Erro de API: {:?}", e))?;
    api.set_signer(pair.clone());

    // Executar o comando
    match cli.command {
        Commands::Balance { address } => {
            let address = if let Some(addr) = address {
                addr
            } else {
                pair.public().to_ss58check()
            };
            
            println!("Consultando saldo para {}", address);
            // Implementar consulta de saldo
            // ...
        }
        Commands::Transfer { to, amount } => {
            println!("Transferindo {} CVX para {}", amount, to);
            // Implementar transferência
            // ...
        }
        Commands::Mine { threads } => {
            println!("Iniciando mineração com {} threads", threads);
            // Implementar mineração
            // ...
        }
        Commands::BlockInfo { block } => {
            let block_ref = if let Some(b) = block {
                b
            } else {
                "latest".to_string()
            };
            
            println!("Consultando informações do bloco {}", block_ref);
            // Implementar consulta de bloco
            // ...
        }
        Commands::DeployContract { contract, value, args } => {
            println!("Implantando contrato de {:?} com valor {}", contract, value);
            // Implementar implantação de contrato
            // ...
        }
        Commands::CallContract { address, method, value, args } => {
            println!("Chamando método {} do contrato {} com valor {}", method, address, value);
            // Implementar chamada de contrato
            // ...
        }
        Commands::GenerateKey { output } => {
            println!("Gerando novo par de chaves em {:?}", output);
            let new_pair = sr25519::Pair::generate();
            let seed = new_pair.to_string();
            fs::write(output, seed)?;
            println!("Chave gerada com sucesso. Endereço: {}", new_pair.public().to_ss58check());
        }
    }

    Ok(())
}
