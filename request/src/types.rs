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


#[derive(Clone, Deserialize)]
pub struct Empty;

impl FromRequest<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}


impl FromRouteParams<Empty> for Empty  {
    fn from_request<'a>(req: &'a Request) -> IronResult<T> {
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