CREATE TABLE clients (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    firstname VARCHAR(50) NOT NULL,
    lastname VARCHAR(50) NOT NULL,
    document_number VARCHAR(50) NOT NULL,
    PRIMARY KEY (id)
);
