use uuid::Uuid;
use crate::domain::clients::client::Client;

pub struct GetAllClientsQueryResponse {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub document_number: String,
}

impl From<Client> for GetAllClientsQueryResponse {
    fn from(item: Client) -> Self {
        GetAllClientsQueryResponse {
            id: *item.id(),
            firstname: item.firstname().to_string(),
            lastname: item.lastname().to_string(),
            document_number: item.document_number().to_string(),
        }
    }
}
