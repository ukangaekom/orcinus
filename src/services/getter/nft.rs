use alloy::{
    primitives::{Address},
    providers::Provider,
    sol
};
use std::str::FromStr;
use crate::connection::provider::{init_sei_mainnet, init_sei_testnet};




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



pub async fn get_nft_total_supply_testnet(nft_address:&str)-> String {

    let provider = init_sei_testnet().await; 
    
    let token_addr = Address::from_str(nft_address).expect("REASON");
    let erc721 = IERC721::new(token_addr,provider.clone());
    let name = erc721.name().call().await.unwrap();
    let symbol = erc721.symbol().call().await.unwrap();
    let totalsupply = erc721.totalSupply().call().await.unwrap();


    format!("NFT collection name {:#?}, bearing the symbol: {:#?} has a total supply of {:#?}",name,symbol,totalsupply)

}



pub async fn get_nft_total_supply_mainnet(nft_address:&str) ->String  {
    
    let provider = init_sei_mainnet().await; 

    
   let token_addr = Address::from_str(nft_address).expect("REASON");
    let erc721 = IERC721::new(token_addr,provider.clone());
    let name = erc721.name().call().await.unwrap();
    let symbol = erc721.symbol().call().await.unwrap();
    let totalsupply = erc721.totalSupply().call().await.unwrap();


    format!("NFT collection name {:#?}, bearing the symbol: {:#?} has a total supply of {:#?}",name,symbol,totalsupply)

}



pub async fn get_nft_details_testnet(nft_address:&str)-> String  {

    let provider = init_sei_testnet().await;  

    let token_addr = Address::from_str(nft_address).expect("REASON");
    let erc721 = IERC721::new(token_addr,provider.clone());
    let name = erc721.name().call().await.unwrap();
    let symbol = erc721.symbol().call().await.unwrap();
    let totalsupply = erc721.totalSupply().call().await.unwrap();



    format!("The NFT name is {:#?} with name {:#?} have a total supply of {:#?}",symbol, name,totalsupply )

}



pub async fn get_nft_details_mainnet(nft_address:&str)-> String  {

    let provider = init_sei_mainnet().await;  

    let token_addr = Address::from_str(nft_address).expect("REASON");
    let erc721 = IERC721::new(token_addr,provider.clone());
    let name = erc721.name().call().await.unwrap();
    let symbol = erc721.symbol().call().await.unwrap();
    let totalsupply = erc721.totalSupply().call().await.unwrap();



    format!("The NFT name is {:#?} with name {:#?} have a total supply of {:#?}",symbol, name,totalsupply )

}




pub async fn get_nft_balance_testnet(nft_address:&str, wallet_address: &str)-> String  {

    let provider = init_sei_testnet().await; 

    let token_addr = Address::from_str(nft_address).expect("REASON");
    let wallet_addr = Address::from_str(wallet_address).expect("REASON");
    let erc721 = IERC721::new(token_addr,provider.clone());
    let name = erc721.name().call().await.unwrap();
    let symbol = erc721.symbol().call().await.unwrap();
    let balance = erc721.balanceOf(wallet_addr).call().await.unwrap();

    format!("The wallet {:#?} has an NFT balance of {:#?} in {:#?}{:#?}",wallet_addr,balance,name,symbol)



}



pub async fn get_nft_balance_mainnet(nft_address:&str, wallet_address: &str) -> String {

    let provider = init_sei_mainnet().await; 

    let token_addr = Address::from_str(nft_address).expect("REASON");
    let wallet_addr = Address::from_str(wallet_address).expect("REASON");
    let erc721 = IERC721::new(token_addr,provider.clone());
    let name = erc721.name().call().await.unwrap();
    let symbol = erc721.symbol().call().await.unwrap();
    let balance = erc721.balanceOf(wallet_addr).call().await.unwrap();

    format!("The wallet {:#?} has an NFT balance of {:#?} in {:#?}{:#?}",wallet_addr,balance,name,symbol)



}
