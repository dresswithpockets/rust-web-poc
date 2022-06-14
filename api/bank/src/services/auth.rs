use tonic::{Request, Response, Status};
use crate::model::auth::auth_server::Auth;
use crate::model::auth::{CreateUserRequest, CreateUserResponse, GrantResponse, RefreshBody};

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserResponse>, Status> {
        todo!()
    }

    async fn handshake(&self, request: Request<RefreshBody>) -> Result<Response<RefreshBody>, Status> {
        todo!()
    }

    async fn grant(&self, request: Request<RefreshBody>) -> Result<Response<GrantResponse>, Status> {
        todo!()
    }

    async fn invalidate(&self, request: Request<RefreshBody>) -> Result<Response<()>, Status> {
        todo!()
    }
}