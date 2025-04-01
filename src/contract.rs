use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_std::types::Token;
use stellar_interchain_token_service::InterchainTokenServiceClient;

use crate::storage_types::DataKey;

#[contract]
pub struct ItsApp;

#[contractimpl]
impl ItsApp {
    /// Initialize the contract with the ITS service address
    pub fn initialize(env: &Env, its_address: Address) {
        env.storage().instance().set(&DataKey::InterchainTokenService, &its_address);
    }

    /// Get the ITS client
    fn its_client(env: &Env) -> InterchainTokenServiceClient {
        let its_address = env.storage().instance().get(&DataKey::InterchainTokenService).unwrap();
        InterchainTokenServiceClient::new(env, &its_address)
    }

    /// Check if a chain is trusted
    pub fn is_trusted_chain(env: &Env, chain: String) -> bool {
        let its = Self::its_client(env);
        its.is_trusted_chain(&chain)
    }

    /// Deploy a new interchain token on Stellar
    pub fn deploy_token(
        env: &Env,
        caller: Address,
        salt: BytesN<32>,
        name: String,
        symbol: String,
        decimals: u32,
        initial_supply: i128,
    ) -> BytesN<32> {
        caller.require_auth();
        
        let its = Self::its_client(env);
        
        // Create token metadata
        let token_metadata = TokenMetadata {
            name,
            symbol,
            decimal: decimals,
        };
        
        // Deploy the token - direct call
        let token_id = its.deploy_interchain_token(
            &caller,
            &salt,
            &token_metadata,
            &initial_supply,
            &None, // No additional minter
        );
        
        // Store the token ID and salt for reference
        env.storage().instance().set(&DataKey::TokenId, &token_id);
        env.storage().instance().set(&DataKey::Salt, &salt);
        
        token_id
    }
    
    /// Deploy the token to another blockchain
    pub fn deploy_remote_token(
        env: &Env,
        caller: Address,
        destination_chain: String,
        gas_token_address: Address,
        gas_amount: i128,
    ) -> BytesN<32> {
        caller.require_auth();
        
        let its = Self::its_client(env);
        
        // Get stored salt
        let salt = env.storage().instance().get(&DataKey::Salt).unwrap_or_else(|| {
            BytesN::from_array(env, &[0; 32])
        });
        
        // Prepare gas token
        let gas_token = Some(Token {
            address: gas_token_address,
            amount: gas_amount,
        });
        
        // Deploy to remote chain - direct call
        let token_id = its.deploy_remote_interchain_token(
            &caller,
            &salt,
            &destination_chain,
            &gas_token,
        );
        
        token_id
    }
    
    /// Transfer tokens to another blockchain
    pub fn transfer_tokens(
        env: &Env,
        caller: Address,
        token_id: BytesN<32>,
        destination_chain: String,
        destination_address: Bytes,
        amount: i128,
        gas_token_address: Address,
        gas_amount: i128,
    ) {
        caller.require_auth();
        
        let its = Self::its_client(env);
        
        // Basic validation
        if amount <= 0 {
            panic!("Invalid amount");
        }
        
        if destination_address.len() == 0 {
            panic!("Invalid destination address");
        }
        
        // Prepare gas token
        let gas_token = Some(Token {
            address: gas_token_address,
            amount: gas_amount,
        });
        
        // Send the tokens cross-chain - direct call
        its.interchain_transfer(
            &caller,
            &token_id,
            &destination_chain,
            &destination_address,
            &amount,
            &None, // No additional data
            &gas_token,
        );
    }
    
    /// Register an existing Stellar token for cross-chain use
    pub fn register_existing_token(
        env: &Env,
        caller: Address,
        token_address: Address,
    ) -> BytesN<32> {
        caller.require_auth();
        
        let its = Self::its_client(env);
        
        // Register the token - direct call
        let token_id = its.register_canonical_token(&token_address);
        
        // Store the token ID for reference
        env.storage().instance().set(&DataKey::TokenId, &token_id);
        
        token_id
    }
    
    /// Get the currently stored token ID
    pub fn get_token_id(env: &Env) -> BytesN<32> {
        env.storage().instance().get(&DataKey::TokenId).unwrap()
    }
}