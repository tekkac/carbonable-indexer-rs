pub mod badge;
pub mod event_source;
pub mod farming;
pub mod minter;
pub mod model;
pub mod offseter;
pub mod payment;
pub mod portfolio;
pub mod project;
pub mod uri;
pub mod yielder;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use starknet::{
    core::types::FieldElement,
    providers::{
        jsonrpc::{
            models::{BlockId, BlockTag},
            HttpTransport, JsonRpcClient,
        },
        SequencerGatewayProvider,
    },
};
use std::sync::Arc;
use thiserror::Error;
use url::Url;

use self::model::ModelError;

#[derive(Error, Debug)]
pub enum SequencerError {
    #[error("environment variable 'NETWORK' not provided")]
    NoEnvProvided,
    #[error("environment variable 'SEQUENCER_DOMAIN' not provided")]
    NoSequencerDomainProvided,
    #[error("environment variable 'JUNO_API_KEY' not provided")]
    NoJunoApiKeyProvided,
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    ReqwestClientError(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),
}

pub enum StarknetEnv {
    Mainnet,
    Goerli,
    Goerli2,
    Sepolia,
    Local,
}

impl From<String> for StarknetEnv {
    fn from(env: String) -> Self {
        match env.as_str() {
            "mainnet" => Self::Mainnet,
            "goerli" => Self::Goerli,
            "sepolia" => Self::Sepolia,
            "goerli2" => Self::Goerli2,
            "local" => Self::Local,
            _ => panic!("Invalid environment"),
        }
    }
}

/// Ensure provided wallet address is 66 char len
/// * wallet_address - [`&mut String`] The wallet address.
///
pub fn ensure_starknet_wallet(wallet_address: &mut String) {
    *wallet_address = wallet_address.to_lowercase();
    if 66 != wallet_address.len() {
        *wallet_address = format!("0x{:0>64}", &wallet_address[2..]);
    }
}

/// Get starknet provider base on "NETWORK" environment variable
/// get_starknet_provider_from_env();
pub fn get_starknet_provider_from_env() -> Result<SequencerGatewayProvider, SequencerError> {
    if let Ok(env) = std::env::var("NETWORK") {
        return get_starknet_provider(env.into());
    }
    Err(SequencerError::NoEnvProvided)
}

/// Get starknet rpc client base on param given "NETWORK" and "SEQUENCER_DOMAIN"
/// get_starknet_rpc_from_env();
pub fn get_starknet_rpc_from_env() -> Result<JsonRpcClient<HttpTransport>, SequencerError> {
    if let Ok(env) = std::env::var("NETWORK") {
        return get_starknet_rpc_client(env.into());
    }
    Err(SequencerError::NoEnvProvided)
}

/// Get starknet provider base on param given:
/// get_starknet_provider(StarknetEnv::Mainnet);
pub fn get_starknet_provider(env: StarknetEnv) -> Result<SequencerGatewayProvider, SequencerError> {
    Ok(match env {
        StarknetEnv::Mainnet => SequencerGatewayProvider::starknet_alpha_mainnet(),
        StarknetEnv::Sepolia => SequencerGatewayProvider::new(
            Url::parse("https://alpha-sepolia.starknet.io/gateway").unwrap(),
            Url::parse("https://alpha-sepolia.starknet.io/feeder_gateway").unwrap(),
        ),
        StarknetEnv::Goerli => SequencerGatewayProvider::starknet_alpha_goerli(),
        StarknetEnv::Goerli2 => SequencerGatewayProvider::starknet_alpha_goerli_2(),
        StarknetEnv::Local => SequencerGatewayProvider::starknet_nile_localhost(),
    })
}

/// Get rpc client from given [`StarknetEnv`]
fn get_starknet_rpc_client(
    env: StarknetEnv,
) -> Result<JsonRpcClient<HttpTransport>, SequencerError> {
    let sequencer_domain = get_sequencer_domain(&env)?;
    let juno_api_key = match std::env::var("JUNO_API_KEY") {
        Ok(k) => k,
        Err(_) => return Err(SequencerError::NoJunoApiKeyProvided),
    };
    let mut headers = HeaderMap::new();
    headers.insert("x-apikey", HeaderValue::from_str(&juno_api_key)?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(JsonRpcClient::new(HttpTransport::new_with_client(
        Url::parse(&sequencer_domain)?,
        client,
    )))
}

/// Get sequencer from given [`StarknetEnv`] variable
fn get_sequencer_domain(env: &StarknetEnv) -> Result<String, SequencerError> {
    if let Ok(domain) = std::env::var("SEQUENCER_DOMAIN") {
        let subdomain = match env {
            StarknetEnv::Mainnet => "starknet-mainnet",
            StarknetEnv::Sepolia => "starknet-sepolia",
            StarknetEnv::Goerli => "starknet-goerli",
            StarknetEnv::Goerli2 => "starknet-goerli2",
            StarknetEnv::Local => "http://localhost:3000",
        };

        return Ok(domain.replace("DOMAIN", subdomain));
    }
    Err(SequencerError::NoSequencerDomainProvided)
}

/// Get proxy class abi
/// * implementation_hash - contract address
pub async fn get_proxy_abi(
    provider: Arc<JsonRpcClient<HttpTransport>>,
    implementation_hash: FieldElement,
) -> Result<serde_json::Value, ModelError> {
    let res = provider
        .get_class_at(&BlockId::Tag(BlockTag::Pending), implementation_hash)
        .await?;
    match res {
        starknet::providers::jsonrpc::models::ContractClass::Sierra(c) => {
            Ok(serde_json::to_value(c.abi)?)
        }
        starknet::providers::jsonrpc::models::ContractClass::Legacy(c) => {
            Ok(serde_json::to_value(c.abi)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::starknet::ensure_starknet_wallet;

    #[test]
    fn test_ensure_wallet() {
        let mut wallet =
            String::from("0x7a108d65e75742c7a0d149e97622c27ad05dec93fd5e952f1d53424128a9ee");
        ensure_starknet_wallet(&mut wallet);
        let expected = "0x007a108d65e75742c7a0d149e97622c27ad05dec93fd5e952f1d53424128a9ee";

        assert_eq!(expected.to_owned(), wallet);

        let mut wallet =
            String::from("0x63675fA1ECEa10063722E61557ED7f49ED2503D6Cdd74F4B31E9770B473650C");
        ensure_starknet_wallet(&mut wallet);
        let expected = "0x063675fa1ecea10063722e61557ed7f49ed2503d6cdd74f4b31e9770b473650c";

        assert_eq!(expected.to_owned(), wallet);

        let mut wallet =
            String::from("0x8d65e75742c7a0d149e97622c27ad05dec93fd5e952f1d53424128a9ee");
        ensure_starknet_wallet(&mut wallet);
        let expected = "0x0000008d65e75742c7a0d149e97622c27ad05dec93fd5e952f1d53424128a9ee";

        assert_eq!(expected.to_owned(), wallet);
    }
}
