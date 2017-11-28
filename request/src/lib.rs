extern crate iron;

pub use types;

#[derive(Debug)]
pub enum ClientError {
    MissingRouteParam(String),
    Other
}

impl ::std::error::Error for ClientError {
    fn description(&self) -> &str {
        match *self {
            ClientError::MissingRouteParam(ref msg) => msg,
            _ => "Could not save file"
        }
    }
}

impl ::std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "TODO: Oh no, something bad went down")
    }
}

pub trait FromRequest<T> : Send + Sync + 'static {
    fn from_request<'a>(req: &'a Request) -> IronResult<T> ;
}

pub trait FromRouteParams<T>: Send + Sync + 'static {
    fn from_params<'a>(params: &::router::Params) -> IronResult<T>;
    fn from_request<'a>(req: &'a Request) -> IronResult<T> {
        Self::from_params(req.extensions.get::<::router::Router>().unwrap())
    }
}

pub trait FromUrlEncoded<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &::iron::Request) -> IronResult<T>;
}

pub trait FromBodyParser<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &'a mut::iron::Request) -> IronResult<T>;
}

pub trait FromQueryParams<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &::iron::Request) -> IronResult<T>;
}

pub struct SimpleRequest<R, Q, B, S>
{
    pub route_params: R,
    pub query_params: Q,
    pub body: B,
    pub extra: S,
}

impl<R, Q, B, S> SimpleRequest<R, Q, B, S>
    where R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
          S: FromRequest<S>,
{
    fn from_request<'a>(req: &'a mut Request) -> IronResult<Self> {
        let route_params = match R::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };
        let query_params = match Q::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };
        let body = match B::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };
        let extra = match S::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };


        Ok(SimpleRequest {
            route_params,
            query_params,
            body,
            extra,
        })
    }
}


#[derive(Clone, Deserialize)]
pub struct Empty;

impl FromRequest<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}


impl FromRouteParams<Empty> for Empty  {
    fn from_params<'a>(_: &::router::Params) -> IronResult<Empty> {
        Ok(Empty)
    }
}

impl FromUrlEncoded<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}

impl FromBodyParser<Empty> for Empty {
    fn from_request<'a>(_: &'a mut Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}

impl FromQueryParams<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
