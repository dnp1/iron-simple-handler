use ::RequestRouteParams;
use ::RequestBody;
use ::RequestQueryParams;
use ::RequestSession;
use ::SimpleResult;
use iron::Request;


//#[derive(Clone, Deserialize)]
pub struct Ignore;

impl RequestSession for Ignore {
    type Services = ();
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore> {
        return Ok(Ignore)
    }
}


impl RequestRouteParams for Ignore {
    type Services = ();
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore> {
        return Ok(Ignore)
    }
}

impl RequestBody for Ignore {
    type Services = ();
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore> {
        return Ok(Ignore)
    }
}

impl RequestQueryParams for Ignore {
    type Services = ();
    #[inline]
    fn from_request<'a>(req: &mut Request, services: &Self::Services) -> SimpleResult<Ignore> {
        return Ok(Ignore)
    }
}