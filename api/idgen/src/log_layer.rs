use std::fmt;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LogLayer {
    target: &'static str,
}

impl LogLayer {
    pub fn new(target: &'static str) -> Self {
        Self {
            target,
        }
    }
}

// this is an implementation of a tower layer. Layer impls provide a service to the tower stack.
// Layers are distinct from tonic services - a Layer intercepts a request through the tonic
// transport, does some processing, may or may not call into the next service, and then returns a
// response. The simplest layer calls the next service and returns its response. Layers are similar
// to Middleware in other web frameworks. "Middleware" and "Layer" are interchangeable in this
// context.
impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service
        }
    }
}

// This service implements the Log behavior
#[derive(Clone)]
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

// this is an implementation of a tower service which logs every request passed into it. A tower
// service is an independent concept from a tower layer. See LogLayer for more details.
impl<S, Request> Service<Request> for LogService<S>
    where
        S: Service<Request>,
        Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}