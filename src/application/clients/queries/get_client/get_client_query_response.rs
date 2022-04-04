use serde::Serialize;

use crate::domain::clients::client::Client;

#[derive(Serialize)]
pub struct GetClientQueryResponse {
    pub firstname: String,
    pub lastname: String,
    pub document_number: String,
}

impl From<Client> for GetClientQueryResponse {
    fn from(item: Client) -> Self {
        GetClientQueryResponse {
            firstname: item.firstname().clone(),
            lastname: item.lastname().clone(),
            document_number: item.document_number().clone(),
        }
    }
}
