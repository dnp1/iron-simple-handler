extern crate iron;

pub mod types;

use iron::Response;
use iron::Request;
use iron::modifier::Modifier;
use std::result::Result;
use iron::status::Status;


#[derive(Debug)]
pub enum ClientError {
    MissingRouteParam(String),
    InvalidRouteParam(String),
    InvalidBody(String),
    UnexpectedEmptyBody(String),
    MissingQueryParam(String),
    InvalidQueryParam(String),
    MissingSession(String),
    InvalidSession(String),
}


impl ClientError {
    fn status(&self) -> iron::status::Status {
        iron::status::BadRequest
    }
    fn description(&self) -> &str {
        match self {
            &ClientError::MissingRouteParam(ref message) => message,
            &ClientError::InvalidRouteParam(ref message) => message,
            &ClientError::UnexpectedEmptyBody(ref message) => message,
            &ClientError::InvalidBody(ref message) => message,
            &ClientError::MissingQueryParam(ref message) => message,
            &ClientError::InvalidQueryParam(ref message) => message,
            &ClientError::MissingSession(ref message) => message,
            &ClientError::InvalidSession(ref message) => message,
        }
    }
}

pub enum ServerError {
    ExtensionNotFound(String),
    PluginNotFound(String),
    ServiceUnavailable(String),
    Other(String),
}

impl ServerError {
    fn status(&self) -> iron::status::Status {
        iron::status::InternalServerError
    }
    fn description(&self) -> &str {
        match self {
            &ServerError::ExtensionNotFound(ref message) => message,
            &ServerError::PluginNotFound(ref message) => message,
            &ServerError::ServiceUnavailable(ref message) => message,
            &ServerError::Other(ref message) => message,
        }
    }
}


pub enum SimpleError {
    Server(ServerError),
    Client(ClientError)
}

impl SimpleError {
    pub fn status(&self) -> iron::status::Status {
        match self {
            &SimpleError::Server(ref e) => e.status(),
            &SimpleError::Client(ref e) => e.status()
        }
    }
    pub fn description(&self) -> &str {
        match self {
            &SimpleError::Server(ref e) => e.description(),
            &SimpleError::Client(ref e) => e.description()
        }
    }
}


pub type SimpleResult<T> = Result<T, SimpleError>;

pub trait RequestRouteParams: Send + Sync + 'static + std::marker::Sized {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Self> where O: Send + Sync + 'static;
}

pub trait RequestQueryParams: Send + Sync + 'static + std::marker::Sized {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Self> where O: Send + Sync + 'static;
}

pub trait RequestBody: Send + Sync + 'static + std::marker::Sized {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Self> where O: Send + Sync + 'static;
}

pub trait RequestSession: Send + Sync + 'static + std::marker::Sized {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Self> where O: Send + Sync + 'static;
}


pub struct SimpleRequest<R, Q, B, S>
{
    pub route_params: R,
    pub query_params: Q,
    pub body: B,
    pub session: S,
}

impl<R, Q, B, S> SimpleRequest<R, Q, B, S>
    where
        R: RequestRouteParams,
        Q: RequestQueryParams,
        B: RequestBody,
        S: RequestSession,
{
    #[inline]
    pub fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Self> where O: Send + Sync + 'static {
        let route_params = match R::from_request(req, services) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let query_params = match Q::from_request(req, services) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let session = match S::from_request(req, services) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let body = match B::from_request(req, services) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };


        Ok(SimpleRequest {
            route_params,
            query_params,
            body,
            session,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
