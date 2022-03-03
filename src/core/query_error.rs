use std::{error, fmt};

#[derive(PartialEq, Debug)]
pub enum QueryError {
    Internal,
    NotFound,
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            QueryError::Internal => "Something bad happened",
            QueryError::NotFound => "No results were found",
        };

        f.write_str(description)
    }
}

impl error::Error for QueryError {}
