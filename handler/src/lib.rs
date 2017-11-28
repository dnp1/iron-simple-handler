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
use iron::status;


pub trait SimpleHandler<R, Q, B, S>
    where R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    fn authenticated(&self) -> bool {
        false
    }
    fn handle(&self, req: &SimpleRequest<R, Q, B, S>) -> IronResult<Response>;
}

pub struct SimpleHandlerBox<T, R, Q, B, S>
    where T: SimpleHandler<R, Q, B, S> + Send + Sync + 'static,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    pub handler: T,
    r: ::std::marker::PhantomData<R>,
    q: ::std::marker::PhantomData<Q>,
    b: ::std::marker::PhantomData<B>,
    s: ::std::marker::PhantomData<S>,
}

impl <T, R, Q, B, S> SimpleHandlerBox<T, R, Q, B, S>
    where T: SimpleHandler<R, Q, B, S> + Send + Sync + 'static,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    pub fn new(handler: T) -> Self {
        SimpleHandlerBox {
            handler,
            r: ::std::marker::PhantomData,
            q: ::std::marker::PhantomData,
            b: ::std::marker::PhantomData,
            s: ::std::marker::PhantomData,
        }
    }
}


impl<T, R, Q, B, S> Handler for SimpleHandlerBox<T, R, Q, B, S>
    where T: SimpleHandler<R, Q, B, S> + Send + Sync + 'static,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let r: SimpleRequest<R, Q, B, S> = match SimpleRequest::from_request(req) {
            Err(s) => {
                return Ok(Response::with((status::BadRequest, "Could not parse body")))
            },
            Ok(val) => val,
        };
        self.handler.handle(&r)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
