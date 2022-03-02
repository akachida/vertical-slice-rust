use uuid::Uuid;

#[derive(Debug, Clone, Queryable)]
pub struct Client {
    id: Uuid,
    firstname: String,
    lastname: String,
    document_number: String,
}

impl Client {
    pub fn new(firstname: String, lastname: String, document_number: String) -> Self {
        Self { id: Uuid::new_v4(), firstname, lastname, document_number }
    }

    pub fn get_id(&self) -> Uuid { self.id }
    pub fn get_firstname(&self) -> String { self.firstname.to_string() }
    pub fn get_lastname(&self) -> String { self.lastname.to_string() }
    pub fn get_document_number(&self) -> String { self.document_number.to_string() }
}
