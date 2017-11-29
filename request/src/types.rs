use ::RequestRouteParams;
use ::RequestBody;
use ::RequestQueryParams;
use ::RequestSession;
use ::SimpleResult;
use iron::Request;


//#[derive(Clone, Deserialize)]
pub struct Ignore;

impl RequestSession<Ignore> for Ignore {
    #[inline]
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}


impl RequestRouteParams<Ignore> for Ignore {
    #[inline]
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}

impl RequestBody<Ignore> for Ignore {
    #[inline]
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}

impl RequestQueryParams<Ignore> for Ignore {
    #[inline]
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}