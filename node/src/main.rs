//! CivicChain Node
//!
//! O nó da CivicChain é responsável por executar a rede blockchain,
//! incluindo a mineração de blocos usando o algoritmo YesPower.

#![warn(missing_docs)]

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

fn main() -> sc_cli::Result<()> {
    command::run()
}
