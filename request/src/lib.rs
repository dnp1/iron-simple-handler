extern crate iron;
use iron::Request;
use iron::Response;
use iron::IronResult;
use iron::modifier::Modifier;


pub trait RequestRouteParams<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &'a Request) -> IronResult<T>;
}

pub trait RequestQueryParams<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &::iron::Request) -> IronResult<T>;
}

pub trait RequestBody<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &'a mut::iron::Request) -> IronResult<T>;
}

pub trait RequestSession<T> : Send + Sync + 'static {
    fn from_request<'a>(req: &'a mut Request) -> IronResult<T>;
}


pub struct SimpleRequest<R, Q, B, S>
{
    pub route_params: R,
    pub query_params: Q,
    pub body: B,
    pub session: S,
}

impl<R, Q, B, S> SimpleRequest<R, Q, B, S>
    where R: RequestRouteParams<R>,
          Q: RequestQueryParams<Q>,
          B: RequestBody<B>,
          S: RequestSession<S>,
{
    pub fn from_request<'a>(req: &'a mut Request) -> IronResult<Self> {
        let route_params = match R::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let query_params = match Q::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let session = match S::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let body = match B::from_request(req) {
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
