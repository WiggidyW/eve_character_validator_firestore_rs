use std::fmt;

#[derive(Debug)]
pub enum Error {
    ValidateError(character_validator::Error),
    FirestoreError(firestore::errors::FirestoreError),
    CharactersMissingError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::ValidateError(error) => {
                write!(f, "ValidateError: {:?}", error)
            },
            Error::FirestoreError(error) => {
                write!(f, "FirestoreError: {}", error)
            },
            Error::CharactersMissingError => {
                write!(f, "CharactersMissingError")
            },
        }
    }
}

impl From<character_validator::Error> for Error {
    fn from(error: character_validator::Error) -> Self {
        Error::ValidateError(error)
    }
}

impl From<firestore::errors::FirestoreError> for Error {
    fn from(error: firestore::errors::FirestoreError) -> Self {
        Error::FirestoreError(error)
    }
}
