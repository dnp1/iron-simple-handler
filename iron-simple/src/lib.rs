extern crate iron;

mod error;
mod handler;

pub use error::*;
pub use handler::*;

use iron::Request;
use iron::Response;
use iron::IronResult;
use std::result::Result;


pub type SimpleResult<T> = Result<T, SimpleError>;

pub trait SimpleErrorTransformer: Send + Sync + 'static {
    fn transform(&self, err: SimpleError) -> IronResult<Response>;
}

pub trait FromIronRequest<O>: std::marker::Sized
    where O: Send + Sync + 'static
{
    fn from_request<'a>(req: &mut Request, services: &O) -> SimpleResult<Self>;
}


impl<O> FromIronRequest<O> for () where
    O: Send + Sync + 'static,
{
    fn from_request<'a>(_: &mut Request, _: &O) -> SimpleResult<Self> {
        Ok(())
    }
}

impl<T, O> FromIronRequest<O> for (T, ) where
    O: Send + Sync + 'static,
    T: FromIronRequest<O>
{
    fn from_request<'a>(req: &mut Request, services: &O) -> SimpleResult<Self> {
        match T::from_request(req, services) {
            Err(err) => Err(err),
            Ok(val) => Ok((val, ))
        }
    }
}

impl<T, T1, O> FromIronRequest<O> for (T, T1) where
    O: Send + Sync + 'static,
    T: FromIronRequest<O>,
    T1: FromIronRequest<O>

{
    fn from_request<'a>(req: &mut Request, services: &O) -> SimpleResult<Self> {
        let v = match T::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };
        let v1 = match T1::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };

        Ok((v, v1))
    }
}

impl<T, T1, T2, O> FromIronRequest<O> for (T, T1, T2) where
    O: Send + Sync + 'static,
    T: FromIronRequest<O>,
    T1: FromIronRequest<O>,
    T2: FromIronRequest<O>

{
    fn from_request<'a>(req: &mut Request, services: &O) -> SimpleResult<Self> {
        let v = match T::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };
        let v1 = match T1::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };
        let v2 = match T2::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };

        Ok((v, v1, v2))
    }
}

impl<T, T1, T2, T3, O> FromIronRequest<O> for (T, T1, T2, T3) where
    O: Send + Sync + 'static,
    T: FromIronRequest<O>,
    T1: FromIronRequest<O>,
    T2: FromIronRequest<O>,
    T3: FromIronRequest<O>,

{
    fn from_request<'a>(req: &mut Request, services: &O) -> SimpleResult<Self> {
        let v = match T::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };
        let v1 = match T1::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };
        let v2 = match T2::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };
        let v3 = match T3::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(val) => val
        };

        Ok((v, v1, v2, v3))
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
