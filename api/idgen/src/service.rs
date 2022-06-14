use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::model::id_gen_server::{IdGen};
use crate::model::{IdRequest, IdResponse};

use snowflake::generator::{IdErr, IdGenerator};

#[derive(Debug)]
pub struct IdGenService<I: IdGenerator> {
    generator: Arc<I>,
}

impl<I: IdGenerator> IdGenService<I> {
    pub fn new(generator: Arc<I>) -> Self {
        IdGenService {
            generator,
        }
    }
}

#[tonic::async_trait]
impl<I: 'static + IdGenerator> IdGen for IdGenService<I> {
    async fn get_next_id(&self, _: Request<IdRequest>) -> Result<Response<IdResponse>, Status> {
        match self.generator.get_id() {
            Ok(value) => Ok(Response::new(IdResponse { uuid: format!("{:0>18}", value).into() })),
            Err(IdErr::NonMonotonic) => Err(Status::new(Code::FailedPrecondition, "Clock source not monotonic."))
        }
    }
}