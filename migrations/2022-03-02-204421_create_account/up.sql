CREATE TABLE accounts (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    branch INT NOT NULL,
    number INT NOT NULL,
    suffix INT NOT NULL,
    type VARCHAR(255) NOT NULL,
    client_id uuid NOT NULL,

    CONSTRAINT client_account_FK FOREIGN KEY (client_id) REFERENCES clients(id)
        ON DELETE NO ACTION,

    PRIMARY KEY (id)
);
