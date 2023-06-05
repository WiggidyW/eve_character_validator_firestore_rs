use crate::{
    firestore::get_characters,
    Error,
};

use character_validator::{Response, JWKS};

pub async fn validate<
    C: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
>(
    firestore_client: &firestore::FirestoreDb,
    collection_name: &str,
    document_id: &str,
    hyper_client: &hyper::client::Client<C, hyper::body::Body>,
    jwks: &JWKS,
    refresh_token: &str,
    client_id: &str,
) -> Result<Response, Error> {
    let characters = get_characters(
        firestore_client,
        collection_name,
        document_id,
    ).await?;
    Ok(
        character_validator::validate(
            hyper_client,
            jwks,
            refresh_token,
            client_id,
            characters.into_iter(),
        ).await?
    )
}
