use mongodb::error::Error as MongoError;
use std::fmt::Debug;
use teloxide::RequestError;

#[derive(Debug)]
pub enum CommandError {
    Request(RequestError),
    Mongo(MongoError),
    Custom(String),
}
impl From<RequestError> for CommandError {
    fn from(err: RequestError) -> Self {
        Self::Request(err)
    }
}
impl From<MongoError> for CommandError {
    fn from(err: MongoError) -> Self {
        Self::Mongo(err)
    }
}

pub type CommandResult<T> = Result<T, CommandError>;
