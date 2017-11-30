extern crate iron;
extern crate request;

use iron::Request;
use iron::Response;
use iron::IronResult;
use iron::Handler;
use request::SimpleRequest;
use request::RequestRouteParams;
use request::RequestQueryParams;
use request::RequestBody;
use request::RequestSession;
use request::SimpleResult;

pub trait FromIronRequest: std::marker::Sized {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Self> where O: Send + Sync + 'static;
}

impl<R, Q, B, S> FromIronRequest for SimpleRequest<R, Q, B, S>
    where
        R: RequestRouteParams,
        Q: RequestQueryParams,
        B: RequestBody,
        S: RequestSession,
{
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Self> where O: Send + Sync + 'static {
        Self::from_request(req, services)
    }
}


pub trait SimpleErrorTransformer: Send + Sync + 'static {
    fn transform(&self, err: request::SimpleError) -> IronResult<Response>;
}

pub trait SimpleHandler: Send + Sync + 'static
{
    type Request: FromIronRequest;

    fn handle(&self, req: &Self::Request) -> IronResult<Response>;

    fn handler<O: Send + Sync + 'static, E: SimpleErrorTransformer>(self, services: O, error_transformer: E) -> SimpleHandlerBox<Self, O, E> where Self: std::marker::Sized {
        SimpleHandlerBox::new(self, services, error_transformer)
    }
}

pub struct SimpleHandlerBox<T, O, E>
    where T: SimpleHandler,
          E: SimpleErrorTransformer,
{
    pub handler: T,
    pub services: O,
    pub error_transformer: E,
}

impl<T, O, E> SimpleHandlerBox<T, O, E>
    where T: SimpleHandler,
          E: SimpleErrorTransformer,
{
    pub fn new(handler: T, services: O, error_transformer: E) -> Self {
        SimpleHandlerBox {
            handler,
            services,
            error_transformer
        }
    }
}

impl<T, O, E> Handler for SimpleHandlerBox<T, O, E>
    where T: SimpleHandler,
          O: 'static + Send + Sync,
          E: SimpleErrorTransformer,
{
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let r: T::Request = match T::Request::from_request(req, &self.services) {
            Err(err) => {
                return self.error_transformer.transform(err);
            }
            Ok(val) => val,
        };
        let resp = match self.handler.handle(&r) {
            Err(e) => return Err(e),
            Ok(data) => data,
        };
        return Ok(resp);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
