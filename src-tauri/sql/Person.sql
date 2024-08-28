-- name: create_person_table
-- Creates the person table
CREATE TABLE Person (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
)

-- name: insert_person
-- Inserts a person into the person table
-- param: name: &str - name of person
INSERT INTO Person (name)
    VALUES (:name)

-- name: get_all_people?
-- Get's all people from person table
SELECT id, name FROM Person