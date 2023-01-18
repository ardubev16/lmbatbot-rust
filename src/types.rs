use mongodb::error::Error as MongoError;
use teloxide::RequestError;

#[derive(Debug)]
pub enum HandlerError {
    Request(RequestError),
    Mongo(MongoError),
    Custom(String),
}
impl From<RequestError> for HandlerError {
    fn from(err: RequestError) -> Self {
        Self::Request(err)
    }
}
impl From<MongoError> for HandlerError {
    fn from(err: MongoError) -> Self {
        Self::Mongo(err)
    }
}

pub type HandlerResult = Result<(), HandlerError>;
