use iron;

#[derive(Debug)]
pub enum ClientError {
    MissingRouteParam(String),
    InvalidRouteParam(String),
    InvalidBody(String),
    UnexpectedEmptyBody(String),
    MissingQueryParam(String),
    InvalidQueryParam(String),
    MissingSession(String),
    InvalidSession(String),
}


impl ClientError {
    fn status(&self) -> iron::status::Status {
        iron::status::BadRequest
    }
    fn description(&self) -> &str {
        match self {
            &ClientError::MissingRouteParam(ref message) => message,
            &ClientError::InvalidRouteParam(ref message) => message,
            &ClientError::UnexpectedEmptyBody(ref message) => message,
            &ClientError::InvalidBody(ref message) => message,
            &ClientError::MissingQueryParam(ref message) => message,
            &ClientError::InvalidQueryParam(ref message) => message,
            &ClientError::MissingSession(ref message) => message,
            &ClientError::InvalidSession(ref message) => message,
        }
    }
}

pub enum ServerError {
    ExtensionNotFound(String),
    PluginNotFound(String),
    ServiceUnavailable(String),
    Other(String),
}

impl ServerError {
    fn status(&self) -> iron::status::Status {
        iron::status::InternalServerError
    }
    fn description(&self) -> &str {
        match self {
            &ServerError::ExtensionNotFound(ref message) => message,
            &ServerError::PluginNotFound(ref message) => message,
            &ServerError::ServiceUnavailable(ref message) => message,
            &ServerError::Other(ref message) => message,
        }
    }
}

pub enum SimpleError {
    Server(ServerError),
    Client(ClientError)
}

impl SimpleError {
    pub fn status(&self) -> iron::status::Status {
        match self {
            &SimpleError::Server(ref e) => e.status(),
            &SimpleError::Client(ref e) => e.status()
        }
    }
    pub fn description(&self) -> &str {
        match self {
            &SimpleError::Server(ref e) => e.description(),
            &SimpleError::Client(ref e) => e.description()
        }
    }
}