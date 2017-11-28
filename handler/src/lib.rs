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

pub trait SimpleDataManager<O>: Send + Sync + 'static {
    fn get(&self, req: & mut Request) -> IronResult<O>;
    fn set(&self, resp: Response, o: O) -> IronResult<Response>;
}

pub trait SimpleHandler: Send + Sync + 'static
{
    fn handle<R, Q, B, S>(&self, req: &SimpleRequest<R, Q, B, S>) -> IronResult<Response>
        where R: RequestRouteParams<R>,
              Q: RequestQueryParams<Q>,
              B: RequestBody<B>,
              S: RequestSession<S>;
}

pub struct SimpleHandlerBox<T, M, O, R, Q, B, S>
    where T: SimpleHandler,
          M: SimpleDataManager<O>,
{
    pub handler: T,
    pub data_manager: M,
    o: ::std::marker::PhantomData<O>,
    r: ::std::marker::PhantomData<R>,
    q: ::std::marker::PhantomData<Q>,
    b: ::std::marker::PhantomData<B>,
    s: ::std::marker::PhantomData<S>,

}

impl<T, M, O, R, Q, B, S> SimpleHandlerBox<T, M, O, R, Q, B, S>
    where T: SimpleHandler,
          M: SimpleDataManager<O>,
{
    pub fn new(handler: T, data_manager: M) -> Self {
        SimpleHandlerBox {
            handler,
            data_manager,
            o: ::std::marker::PhantomData,
            r: ::std::marker::PhantomData,
            q: ::std::marker::PhantomData,
            b: ::std::marker::PhantomData,
            s: ::std::marker::PhantomData,
        }
    }
}

impl<T, M, O, R, Q, B, S> Handler for SimpleHandlerBox<T, M, O, R, Q, B, S>
    where T: SimpleHandler,
          M: SimpleDataManager<O>,
          O : 'static + Send + Sync,
          R : RequestRouteParams<R>,
          Q : RequestQueryParams<Q>,
          B : RequestBody<B>,
          S : RequestSession<S>,
{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut o = match self.data_manager.get(req) {
            Err(err) => return Err(err),
            Ok(value) => value,
        };

        let r: SimpleRequest<R, Q, B, S> = match SimpleRequest::from_request(&mut o) {
            Err(err) => {
                return Err(err);
            }
            Ok(val) => val,
        };
        let resp = match self.handler.handle(&r)  {
            Err(e) => return Err(e),
            Ok(data) => data,
        };
        return self.data_manager.set(resp, o);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
