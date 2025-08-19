use alloy::{
    primitives::{Address},
    sol
};
use std::str::FromStr;
use crate::connection::provider::{init_sei_mainnet, init_sei_testnet};



// Definition of Contracts
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


// FUNCTIONS

pub async fn get_token_total_supply_testnet(token_address:&str) -> String {
     
    let provider = init_sei_testnet().await; 
    let token_addr =Address::from_str(token_address).expect("REASON");
    let erc20 = IERC20::new(token_addr,provider.clone());
    let name = erc20.name().call().await;
    
    match name {
        Ok(name) => {
            let symbol = erc20.symbol().call().await.unwrap();
            let totalsupply = erc20.totalSupply().call().await.unwrap();


            return format!(
                "token name {:#?}, symbol: {:#?} and a total supply of {:#?}",name,symbol,totalsupply
            );

        }
        Err(_e) => {

            return format!(
                "The token address {:#?} doesn't exist on sei, check if it's an NFT or wallet.
                If it doesn't still exist, then its likely on another chain or doesn't exist",
            token_addr)


        }
    }
}


pub async fn get_token_total_supply_mainnet(token_address:&str) -> String {

    let provider = init_sei_mainnet().await; 
    let token_addr = Address::from_str(token_address).expect("REASON");
    let erc20 = IERC20::new(token_addr,provider.clone());
    let name = erc20.name().call().await;

    match name {
        Ok(name) => {
            let symbol = erc20.symbol().call().await.unwrap();
            let totalsupply = erc20.totalSupply().call().await.unwrap();


            return format!(
                "token name {:#?}, symbol: {:#?} and a total supply of {:#?}",name,symbol,totalsupply
            );

        }
        Err(_e) => {

            return format!(
                "The token address {:#?} doesn't exist on sei, check if it's an NFT or wallet.
                If it doesn't still exist, then its likely on another chain or doesn't exist",
            token_addr)


        }
    }
    
}





pub async fn get_token_details_testnet(token_address:&str) -> String {

    let provider = init_sei_testnet().await; 
    let token_addr = Address::from_str(token_address).expect("REASON");
    let erc20 = IERC20::new(token_addr,provider.clone());
    let name = erc20.name().call().await;

    match name {
        Ok(name) => {
            let symbol = erc20.symbol().call().await.unwrap();
            let decimal = erc20.decimals().call().await.unwrap();
            let totalsupply = erc20.totalSupply().call().await.unwrap();


            return format!(
                "The token name is {:#?} with name {:#?} having a decimal of {:#?} and total supply of {:#?}",
                symbol, name,decimal,totalsupply );

        }
        Err(_e) => {

            return format!(
                "The token address {:#?} doesn't exist on sei, check if it's an NFT or wallet.
                If it doesn't still exist, then its likely on another chain or doesn't exist",
            token_addr)


        }
    }
    

}


pub async fn get_token_details_mainnet(token_address:&str)-> String {

    let provider = init_sei_mainnet().await;  
    let token_addr = Address::from_str(token_address).expect("REASON");
    let erc20 = IERC20::new(token_addr,provider.clone());
    let name = erc20.name().call().await;

    match name {
        Ok(name) => {
            let symbol = erc20.symbol().call().await.unwrap();
            let decimal = erc20.decimals().call().await.unwrap();
            let totalsupply = erc20.totalSupply().call().await.unwrap();


            return format!(
                "The token name is {:#?} with name {:#?} having a decimal of {:#?} and total supply of {:#?}",
                symbol, name,decimal,totalsupply );

        }
        Err(_e) => {

            return format!(
                "The token address {:#?} doesn't exist on sei, check if it's an NFT or wallet or .
                If it doesn't still exist, then its likely on another chain or doesn't exist",
            token_addr)


        }
    }
    
}




pub async fn get_token_balance_testnet(token_address:&str, wallet_address: &str)-> String {


    let provider = init_sei_testnet().await;  
    let token_addr = Address::from_str(token_address).expect("REASON");
    let wallet_addr = Address::from_str(wallet_address).expect("REASON");
    let erc20 = IERC20::new(token_addr,provider.clone());

    println!("The ERROR occured! {:#?}",erc20);

    let token_decimal = erc20.decimals().call().await;
    
    match token_decimal{
        Ok(token_decimal) =>{
            let decimal: i32 = token_decimal.into();
            let symbol = erc20.symbol().call().await.unwrap();
            let balance: f32 = erc20.balanceOf(wallet_addr).call().await.unwrap().try_into().unwrap();

            let power:f32 = 10.0_f32.powi(decimal).into();
            let token_balance: f32 = balance/power;

            
            return format!(
                "The wallet {:#?} has a token balance of {:#?}{:#?}",wallet_addr,token_balance,symbol
            );

        }

        Err(_e) => {

            return format!(
                "The token address {:#?} does not exist on sei network",token_addr
            );
        }

    }
    
}



pub async fn get_token_balance_mainnet(token_address:&str, wallet_address: &str)-> String {

    let provider = init_sei_mainnet().await; 
    let token_addr = Address::from_str(token_address).expect("REASON");
    let wallet_addr = Address::from_str(wallet_address).expect("REASON");
    let erc20 = IERC20::new(token_addr,provider.clone());

    println!("The ERROR occured! {:#?}",erc20);

    let token_decimal = erc20.decimals().call().await;
    
    match token_decimal{
        Ok(token_decimal) =>{
            let decimal: i32 = token_decimal.into();
            let symbol = erc20.symbol().call().await.unwrap();
            let balance: f32 = erc20.balanceOf(wallet_addr).call().await.unwrap().try_into().unwrap();

            let power:f32 = 10.0_f32.powi(decimal).into();
            let token_balance: f32 = balance/power;

            
            return format!(
                "The wallet {:#?} has a token balance of {:#?}{:#?}",wallet_addr,token_balance,symbol
            );

        }

        Err(_e) => {

            return format!(
                "The token address {:#?} does not exist on sei network",token_addr
            );
        }

    }
    
}
