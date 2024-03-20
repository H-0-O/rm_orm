use mongodb::options::ClientOptions;
use mongodb::Client;

use crate::client::RmORMResult;

use crate::error::RmORMError;

pub async fn create_client_options(
    user_name: &str,
    password: &str,
    host: &str,
    port: &str,
) -> RmORMResult<ClientOptions> {
    let connection_string = format!("mongodb://{}:{}@{}:{}", user_name, password, host, port);
    let client_options = ClientOptions::parse(connection_string).await;
    match client_options {
        Ok(otp) => Ok(otp),
        Err(err) => Err(RmORMError::new(&err.to_string())),
    }
}

pub fn create_client(options: ClientOptions) -> RmORMResult<Client> {
    Ok(Client::with_options(options).unwrap())
}
