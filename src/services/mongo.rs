use mongodb::{options::ClientOptions, Client, Collection};

pub async fn get_connection() -> Client {
    let mut client_options = ClientOptions::parse("mongodb://root:12345678@127.0.0.1:27021")
        .await
        .unwrap();

    client_options.app_name = Some("Test".to_string());

    Client::with_options(client_options).unwrap()
}

pub async fn get_collection(collection_name: &str) -> Collection {
    let client = get_connection().await;

    let db = client.database("test");

    db.collection(collection_name)
}
