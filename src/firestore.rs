use crate::Error;

use serde::Deserialize;

#[derive(Deserialize)]
struct Characters {
    characters: Vec<String>,
}

pub async fn get_characters(
    firestore_client: &firestore::FirestoreDb,
    collection_name: &str,
    document_id: &str,
) -> Result<Vec<String>, Error> {
    Ok(firestore_client
        .fluent()
        .select()
        .by_id_in(collection_name)
        .obj::<Characters>()
        .one(document_id)
        .await?
        .ok_or(Error::CharactersMissingError)?
        .characters
    )
}
