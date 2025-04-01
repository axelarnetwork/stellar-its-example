use stellar_axelar_std::{contracterror, soroban_sdk};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ContractError {
    TrustedChainAlreadySet = 1,
    TrustedChainNotSet = 2,
    InvalidMessageType = 3,
    UntrustedChain = 4, 
    InvalidAmount = 5,
    InvalidDestinationAddress = 6,
    NotHubChain = 7,
    NotHubAddress = 8,
    InvalidTokenId = 9,
    TokenAlreadyRegistered = 10,
    FlowLimitExceeded = 11,
    InvalidDestinationChain = 12,
    InvalidData = 13,
    InvalidTokenName = 14,
    InvalidTokenSymbol = 15,
    InvalidTokenDecimals = 16,
    InvalidInitialSupply = 17,
    InvalidTokenConfig = 18,
}