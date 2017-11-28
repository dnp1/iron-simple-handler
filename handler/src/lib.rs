extern crate iron;

use iron::Request;
use iron::Response;
use iron::IronResult;

pub trait SimpleHandler<R, Q, B, S>
    where R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
          S: FromRequest<S>,
{
    fn authenticated(&self) -> bool {
        false
    }
    fn handle(&self, req: &SimpleRequest<R, Q, B, S>, session: &mut Session) -> IronResult<Response>;
}

pub struct SimpleHandlerBox<T, R, Q, B>
    where T: SimpleHandler<R, Q, B, Empty> + Send + Sync + 'static,
          R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
{
    pub handler: T,
    pub sm: Arc<SessionManager>,
    r: ::std::marker::PhantomData<R>,
    q: ::std::marker::PhantomData<Q>,
    b: ::std::marker::PhantomData<B>,
}

impl <T, R, Q, B> SimpleHandlerBox<T, R, Q, B>
    where T: SimpleHandler<R, Q, B, Empty> + Send + Sync + 'static,
          R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
{
    pub fn new(handler: T, sm: Arc<SessionManager>) -> Self {
        SimpleHandlerBox {
            handler,
            sm,
            r: ::std::marker::PhantomData,
            q: ::std::marker::PhantomData,
            b: ::std::marker::PhantomData,
        }
    }
}


impl<T, R, Q, B> Handler for SimpleHandlerBox<T, R, Q, B>
    where T: SimpleHandler<R, Q, B, Empty> + Send + Sync + 'static,
          R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut session = match self.sm.get_request_session(req) {
            None => return Ok(Response::with((status::Unauthorized, "You must create a session"))),
            Some(session) => {
                if self.handler.authenticated() {
                    if let None = session.user_id {
                        return Ok(Response::with((status::Unauthorized, "You must authenticate with an user")));
                    }
                }
                session
            }
        };

        let r: SimpleRequest<R, Q, B, Empty> = match SimpleRequest::from_request(req) {
            Err(s) => {
                println!("{}", s);
                ::std::io::stdout().flush().unwrap();
                return Ok(Response::with((status::BadRequest, "Could not parse body")))
            },
            Ok(val) => val,
        };

        match self.handler.handle(&r, &mut session) {
            Ok(mut response) => {
                match self.sm.create_session_payload(&mut session) {
                    Err(err) => Ok(Response::with((status::InternalServerError, err.to_string()))),
                    Ok(payload) => {
                        set_cookie(&mut response, &payload);
                        Ok(response)
                    }
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
