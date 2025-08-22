// Defining Modules
mod response;
mod agents;
mod services;
mod tools;
mod connection;

use response::request;
use axum::{
    routing::post,
    Router
};
use tokio::net::TcpListener;





#[tokio::main]
async fn main(){
    dotenv_flow::dotenv_flow().ok();

    // Axum router 
    let router_sei_orcinus: Router = Router::new().route("/sei_orcinus_agent",
    post(request));


    // Define Ip and Port
    let address: &'static str = "0.0.0.0:8080";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();


    println!("Listener on {address}\n");

        
    axum::serve(listener, router_sei_orcinus).await.unwrap();
}




#[cfg(test)]
mod sei_orcinus_test{
    #[test]
    fn test_request(){
        super::request()
    }

}


