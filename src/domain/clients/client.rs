use diesel::Queryable;
use uuid::Uuid;
use crate::schema::clients;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name="clients"]
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

    pub fn id(&self) -> &Uuid { &self.id }

    pub fn firstname(&self) -> &String { &self.firstname }
    pub fn set_firstname(&mut self, value: String) { self.firstname = value; }

    pub fn lastname(&self) -> &String { &self.lastname }
    pub fn set_lastname(&mut self, value: String) { self.lastname = value; }

    pub fn document_number(&self) -> &String { &self.document_number }
    pub fn set_document_number(&mut self, value: String) { self.document_number = value; }
}
