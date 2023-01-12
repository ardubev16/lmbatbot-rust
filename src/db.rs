use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client, Database,
};
use std::env;

pub fn get_db() -> Database {
    let credentials = Credential::builder()
        .username(env::var("MONGO_USERNAME").expect("MONGO_USERNAME not set"))
        .password(env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD not set"))
        .build();

    // TODO: add possibility to use URL for connection
    let client_options = ClientOptions::builder()
        .hosts(vec![ServerAddress::Tcp {
            host: "mongodb".to_string(),
            port: None,
        }])
        .default_database(Some("lmbatbot".to_string()))
        .credential(credentials)
        .build();

    let client = Client::with_options(client_options)
        .expect("Failed to initialize client.");

    client.default_database().unwrap()
}
