use ::RequestRouteParams;
use ::RequestBody;
use ::RequestQueryParams;
use ::RequestSession;
use ::SimpleResult;
use iron::Request;

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


//#[derive(Clone, Deserialize)]
pub struct Ignore;

impl RequestSession<Ignore> for Ignore {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}


impl RequestRouteParams<Ignore> for Ignore {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}

impl RequestBody<Ignore> for Ignore {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}

impl RequestQueryParams<Ignore> for Ignore {
    fn from_request<'a, O>(req: &mut Request, services: &O) -> SimpleResult<Ignore> where O: Send + Sync + 'static {
        return Ok(Ignore)
    }
}