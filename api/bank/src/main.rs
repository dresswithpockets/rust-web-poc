mod services;

pub mod model {
    pub mod auth {
        tonic::include_proto!("auth");
    }
    pub mod user {
        tonic::include_proto!("user");
    }
    pub mod account {
        tonic::include_proto!("account");
    }
    pub mod transaction {
        tonic::include_proto!("transaction");
    }
}

use tokio;
use tonic::transport::Server;

// grpc servers
use model::auth::auth_server::AuthServer;
use model::user::user_server::UserServer;
use model::account::account_server::AccountServer;
use model::transaction::transaction_server::TransactionServer;

// our service implementations
use services::auth::AuthService;
use services::transaction::TransactionService;
use services::account::AccountService;
use services::user::UserService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let address = "[::1]:10000".parse().unwrap();

    let auth_service = AuthService::new();
    let auth_server = AuthServer::new(auth_service);

    let account_service = AccountService::new();
    let account_server = AccountServer::new(account_service);

    let trans_service = TransactionService::new();
    let trans_server = TransactionServer::new(trans_service);

    let user_service = UserService::new();
    let user_server = UserServer::new(user_service);

    Server::builder()
        .add_service(auth_server)
        .add_service(account_server)
        .add_service(trans_server)
        .add_service(user_server)
        .serve(address).await?;

    Ok(())
}
