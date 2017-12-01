use ::RequestRouteParams;
use ::RequestBody;
use ::RequestQueryParams;
use ::RequestSession;
use ::SimpleResult;
use iron::Request;


//#[derive(Clone, Deserialize)]

pub struct Ignore<O>(::std::marker::PhantomData<O>);

impl <O: Send + Sync + 'static> RequestSession for Ignore<O> {
    type Services = O;
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore<O>> {
        return Ok(Ignore(::std::marker::PhantomData))
    }
}


impl <O: Send + Sync + 'static>RequestRouteParams for Ignore<O> {
    type Services = O;
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore<O>> {
        return Ok(Ignore(::std::marker::PhantomData))
    }
}

impl <O: Send + Sync + 'static> RequestBody for Ignore<O> {
    type Services = O;
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore<O>> {
        return Ok(Ignore(::std::marker::PhantomData))
    }
}

impl <O: Send + Sync + 'static>RequestQueryParams for Ignore<O> {
    type Services = O;
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore<O>> {
        return Ok(Ignore(::std::marker::PhantomData))
    }
}