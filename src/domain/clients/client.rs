use diesel::Queryable;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable)]
pub struct Client {
    id: Uuid,
    firstname: String,
    lastname: String,
    document_number: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            firstname: "".to_string(),
            lastname: "".to_string(),
            document_number: "".to_string(),
        }
    }

    pub fn get_id(&self) -> Uuid { self.id }

    pub fn get_firstname(&self) -> &String { &self.firstname }
    pub fn set_firstname(&mut self, value: String) { self.firstname = value; }

    pub fn get_lastname(&self) -> &String { &self.lastname }
    pub fn set_lastname(&mut self, value: String) { self.lastname = value; }

    pub fn get_document_number(&self) -> &String { &self.document_number }
    pub fn set_document_number(&mut self, value: String) { self.document_number = value; }
}
