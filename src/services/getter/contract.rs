use alloy::{
    primitives::{Address},
    providers::Provider,
    sol
};
use std::str::FromStr;
use crate::connection::provider::{init_sei_mainnet, init_sei_testnet};

sol!(
    #[sol(rpc)]
    contract IERC20 {

        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
        function name() returns (string);
        function symbol() external view returns (string memory);
        function totalSupply() external view returns (uint256);
        function decimals() external view returns (uint8);

    }
);




// NFT Standard Contract Interface
sol!(

    #[sol(rpc)]
    contract IERC721{

        function name() returns (string);
        function symbol() external view returns (string memory);
        function tokenURI(uint256 tokenId) external view returns (string memory);

        function totalSupply() external view returns (uint256);
        /// @notice Returns the number of NFTs owned by `owner`.
        function balanceOf(address owner) external view returns (uint256 balance);
        /// @notice Returns the owner of `tokenId`.
        function ownerOf(uint256 tokenId) external view returns (address owner);
        /// @notice Safely transfers `tokenId` from `from` to `to`, checking `to` can handle ERC721.
        function safeTransferFrom(address from, address to, uint256 tokenId) external;
        /// @notice Safe transfer with additional `data`.
        function safeTransferFrom(address from, address to, uint256 tokenId, bytes calldata data) external;
        /// @notice Transfers `tokenId` from `from` to `to` (no safety check).
        function transferFrom(address from, address to, uint256 tokenId) external;
        /// @notice Approves `to` to transfer `tokenId`.
        function approve(address to, uint256 tokenId) external;
        /// @notice Returns the approved address for `tokenId`.
        function getApproved(uint256 tokenId) external view returns (address operator);
        /// @notice Approve or remove `operator` as an operator for the caller.
        function setApprovalForAll(address operator, bool _approved) external;
        /// @notice Returns if `operator` is allowed to manage all of `owner`'s assets.
        function isApprovedForAll(address owner, address operator) external view returns (bool);
    }
);






// CHECKING ADDRESS

pub async fn check_contract_testnet(address: &str) -> String{
    
    let provider = init_sei_testnet().await;

    let addr = Address::from_str(address).expect("REASON");



    return "address".to_string();

}


pub async fn check_contract_mainnet(address: &str) -> String{

    let provider = init_sei_mainnet().await;

    let addr = Address::from_str(address).expect("REASON");

    return "address".to_string();

}


pub async fn check_contract(address: &str) -> String{

    let provider_testnet = init_sei_testnet().await;

    let provider_mainnet = init_sei_mainnet().await;

    let addr = Address::from_str(address).expect("REASON");

    return "contract".to_string();

}



