use serde::Serialize;

use crate::domain::clients::client::Client;

#[derive(Serialize)]
pub struct GetClientQueryResponse {
    pub firstname: String,
    pub lastname: String,
    pub document_number: String,
}

impl From<&Client> for GetClientQueryResponse {
    fn from(item: &Client) -> Self {
        GetClientQueryResponse {
            firstname: item.get_firstname(),
            lastname: item.get_lastname(),
            document_number: item.get_document_number(),
        }
    }
}
