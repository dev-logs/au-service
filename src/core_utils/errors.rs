use thiserror::Error;

#[derive(Error, Debug)]
pub enum OurErrors {
    #[error("Failed to perform database commands")]
    DbExecuteCommandError(),
}
