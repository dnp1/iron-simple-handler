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

pub trait SimpleErrorTransformer: Send + Sync + 'static {
    fn transform(&self, err: request::SimpleError) -> IronResult<Response>;
}

pub trait SimpleHandler<R, Q, B, S>: Send + Sync + 'static
    where R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>
{
    fn handle(&self, req: &SimpleRequest<R, Q, B, S>) -> IronResult<Response>;

    fn handler<O: Send + Sync + 'static, E: SimpleErrorTransformer>(self, services: O, error_transformer: E) -> SimpleHandlerBox<Self, O, R, Q, B, S, E> where Self: std::marker::Sized {
        SimpleHandlerBox::new(self, services, error_transformer)
    }
}

pub struct SimpleHandlerBox<T, O, R, Q, B, S, E>
    where T: SimpleHandler<R, Q, B, S>,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
          E: SimpleErrorTransformer,
{
    pub handler: T,
    pub services: O,
    pub error_transformer: E,
    o: ::std::marker::PhantomData<O>,
    r: ::std::marker::PhantomData<R>,
    q: ::std::marker::PhantomData<Q>,
    b: ::std::marker::PhantomData<B>,
    s: ::std::marker::PhantomData<S>,

}

impl<T, O, R, Q, B, S, E> SimpleHandlerBox<T, O, R, Q, B, S, E>
    where T: SimpleHandler<R, Q, B, S>,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
          E: SimpleErrorTransformer,
{
    pub fn new(handler: T, services: O, error_transformer: E) -> Self {
        SimpleHandlerBox {
            handler,
            services,
            error_transformer,
            o: ::std::marker::PhantomData,
            r: ::std::marker::PhantomData,
            q: ::std::marker::PhantomData,
            b: ::std::marker::PhantomData,
            s: ::std::marker::PhantomData,
        }
    }
}

impl<T, O, R, Q, B, S, E> Handler for SimpleHandlerBox<T, O, R, Q, B, S, E>
    where T: SimpleHandler<R, Q, B, S>,
          O: 'static + Send + Sync,
          R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
          E: SimpleErrorTransformer,
{
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let r: SimpleRequest<R, Q, B, S> = match SimpleRequest::from_request(req, &self.services) {
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
