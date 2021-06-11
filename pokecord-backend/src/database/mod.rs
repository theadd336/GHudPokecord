//! Functions related to accessing the Pokecord database.
//! The first call to any database function will incur an additional
//! runtime penalty, as the database connection must be established.

use crate::models::Pokemon;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    options::{ClientOptions, UpdateOptions},
    Client, Collection,
};
use once_cell::sync::OnceCell;

mod errors;
pub use errors::DatabaseError;

/// Represents the MongoDB host connection.
const MONGO_HOST: &str = "mongodb://admin:secret@database:27017";
/// Represents the main pokecord database.
const POKECORD_DB: &str = "pokecord_db";
/// Represents the collection of pokecord players.
const PLAYER_COLLECTION: &str = "pokecord_players";

/// Represents a MongoDB client. This value is initialized once and then cached.
static CLIENT: OnceCell<Client> = OnceCell::new();

/// Type alias for `Result<T, DatabaseError>`
pub type DBError<T> = Result<T, DatabaseError>;

/// Gets a reference to the MongoDB client, creating one if needed.
async fn get_client_internal() -> &'static Client {
    match CLIENT.get() {
        Some(client) => client,
        None => {
            let client_options = ClientOptions::parse(MONGO_HOST).await.unwrap();
            let client =
                Client::with_options(client_options).expect("Failed to create a MongoDB client.");
            // Safety: If multiple threads try to set at once, only one will
            // succeed. In that event, just discard the error on this thread
            // and use the set value.
            let _ = CLIENT.set(client);
            CLIENT.get().unwrap()
        }
    }
}

/// Helper function to get a reference to the Pokecord player collection
async fn get_collection() -> Collection {
    get_client_internal()
        .await
        .database(POKECORD_DB)
        .collection(PLAYER_COLLECTION)
}

/// Registers a given player ID and starter pokemon for the game.
/// Player IDs must be unique in the database, and this function will return
/// a `PlayerAlreadyExists` error if the player is already registered.
///
/// # Arguments
///
/// * `player_id` - The player ID to register
/// * `starter` - The starter Pokemon to add to the player's pokedex
///
/// # Returns
/// * `()` on success, `DatabaseError` on failure.
pub async fn register_player(player_id: &str, starter: &Pokemon) -> DBError<()> {
    log::info!(
        "Attempting to add player {} and starter {:?}",
        player_id,
        starter
    );
    let collection = get_collection().await;
    let filter = doc! {"player_id": player_id};
    let mut update_options = UpdateOptions::default();
    update_options.upsert = Some(true);
    let update_doc = doc! {
        "$setOnInsert": {
            "player_id": player_id,
            "pokemon": bson::to_bson(&[starter]).unwrap(),
            "buddy": 0
        },
    };
    let result = collection
        .update_one(filter, update_doc, update_options)
        .await?;
    if result.matched_count > 0 {
        log::warn!("The player already exists. Player IDs must be unique.");
        return Err(DatabaseError::PlayerAlreadyExists);
    }
    let id: ObjectId = bson::from_bson(result.upserted_id.unwrap()).unwrap();
    log::info!(
        "Successfully created database entry {} for player {}",
        id,
        player_id
    );
    Ok(())
}
