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


pub trait SimpleHandler<R, Q, B, S>: Send + Sync + 'static where R: RequestRouteParams<R>,
                                                                 Q: RequestQueryParams<Q>,
                                                                 B: RequestBody<B>,
                                                                 S: RequestSession<S>
{
    fn handle(&self, req: &SimpleRequest<R, Q, B, S>) -> IronResult<Response>;
    fn handler<O: Send + Sync + 'static>(self, services: O) -> SimpleHandlerBox<Self, O, R, Q, B, S> where Self: std::marker::Sized {
        SimpleHandlerBox::new(self, services)
    }
}

pub struct SimpleHandlerBox<T, O, R, Q, B, S>
    where T: SimpleHandler<R, Q, B, S>,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    pub handler: T,
    pub services: O,
    o: ::std::marker::PhantomData<O>,
    r: ::std::marker::PhantomData<R>,
    q: ::std::marker::PhantomData<Q>,
    b: ::std::marker::PhantomData<B>,
    s: ::std::marker::PhantomData<S>,

}

impl<T, O, R, Q, B, S> SimpleHandlerBox<T, O, R, Q, B, S>
    where T: SimpleHandler<R, Q, B, S>,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    pub fn new(handler: T, services: O) -> Self {
        SimpleHandlerBox {
            handler,
            services,
            o: ::std::marker::PhantomData,
            r: ::std::marker::PhantomData,
            q: ::std::marker::PhantomData,
            b: ::std::marker::PhantomData,
            s: ::std::marker::PhantomData,
        }
    }
}

impl<T, O, R, Q, B, S> Handler for SimpleHandlerBox<T, O, R, Q, B, S>
    where T: SimpleHandler<R, Q, B, S>,
          O: 'static + Send + Sync,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let r: SimpleRequest<R, Q, B, S> = match SimpleRequest::from_request(req, &self.services) {
            Err(err) => {
                return Err(err);
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
