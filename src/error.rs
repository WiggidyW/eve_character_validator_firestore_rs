pub enum Error {
    ValidateError(character_validator::Error),
    FirestoreError(firestore::errors::FirestoreError),
    CharactersMissingError,
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
