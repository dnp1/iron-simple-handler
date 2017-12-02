use iron::Request;
use iron::Response;
use iron::IronResult;
use iron::Handler;

use super::FromIronRequest;
use super::SimpleErrorTransformer;

pub trait SimpleHandler: Send + Sync + 'static
{
    type Services: Send + Sync + 'static;
    type Request: FromIronRequest<Self::Services>;

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response>;

    #[inline]
    fn handler<E: SimpleErrorTransformer>(self, services: Self::Services, error_transformer: E) -> SimpleHandlerBox<Self, Self::Services, E>
        where Self: ::std::marker::Sized {
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

impl<T, O, E, R> Handler for SimpleHandlerBox<T, O, E>
    where T: SimpleHandler<Request=R, Services=O>,
          R: FromIronRequest<O>,
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
        let resp = match self.handler.handle(r, &self.services) {
            Err(e) => return Err(e),
            Ok(data) => data,
        };
        return Ok(resp);
    }
}

#[cfg(test)]
mod tests {
    use iron::prelude::{Response, IronResult};

    pub struct MyHand;

    use SimpleHandler;
    use super::SimpleErrorTransformer;


    impl SimpleHandler for MyHand {
        type Request = ();
        type Services = ();

        fn handle(&self, _req: Self::Request, _services: &Self::Services) -> IronResult<Response> {
            unimplemented!()
        }
    }

    struct NoTransform;

    impl SimpleErrorTransformer for NoTransform {
        fn transform(&self, _err: SimpleError) -> IronResult<Response> {
            unimplemented!()
        }
    }

    #[test]
    fn it_works() {
        MyHand.handler((), NoTransform);
    }
}