use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use urlencoding::decode;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Person {
    name: String,
    age: u32,
    #[serde(default)]
    drinks: Vec<String>,
}

// Mapa que almacenará la lista de personas
pub type Database = Arc<Mutex<HashMap<String, Person>>>;

pub fn create_database() -> Database {
    Arc::new(Mutex::new(HashMap::new()))
}

//Peticion POST
pub fn handle_post(body: &str, db: &Database) -> String {
    // Intenta deserializar el JSON del cuerpo
    let new_person: Person = match serde_json::from_str(body) {
        Ok(person) => person,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);  // Ver errores en la consola
            return "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data.".to_string();
        }
    };

    let mut db = db.lock().unwrap();

    // Decodificar el nombre al insertar en la base de datos
    let decoded_name = decode(&new_person.name).unwrap_or_else(|_| new_person.name.clone().into()).into_owned();

    if db.contains_key(&decoded_name) {
        return "HTTP/1.1 409 Conflict\r\n\r\nPerson already exists.".to_string();
    }

    db.insert(decoded_name, new_person.clone());

    // Mensaje de confirmación con la persona creada
    let confirmation = serde_json::to_string(&new_person).unwrap();
    format!("HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{}", confirmation)
}

//Peticion GET
pub fn handle_get(request: &str, db: &Database) -> String {
    let name = extract_name_from_request(request);

    let db = db.lock().unwrap();
    if let Some(person) = db.get(&name) {
        let json = serde_json::to_string(&person).unwrap();
        format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", json)
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.".to_string()
    }
}

fn extract_name_from_request(request: &str) -> String {
    let lines: Vec<&str> = request.lines().collect();
    let url = lines[0].split_whitespace().nth(1).unwrap_or("");
    let name = url.trim_start_matches("/person/");

    // Decodificar el nombre directamente a un `String` con `.to_string()` o `.into_owned()` donde sea necesario
    decode(name).unwrap_or_else(|_| name.into()).into_owned()
}

//Peticion PUT
pub fn handle_put(request: &str, body: &str, db: &Database) -> String {
    let name_in_url = extract_name_from_request(request);
    let updated_person: Person = match serde_json::from_str(body) {
        Ok(person) => person,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);  // Ver errores en la consola
            return "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data.".to_string();
        }
    };

    let mut db = db.lock().unwrap();

    // Decodificar el nombre antes de buscarlo en la base de datos
    if db.contains_key(&name_in_url) {
        if updated_person.name != name_in_url {
            db.remove(&name_in_url);  // Eliminamos la entrada antigua
        }
        db.insert(updated_person.name.clone(), updated_person.clone());

        let person_json = serde_json::to_string(&updated_person).unwrap();
        format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", person_json)
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.".to_string()
    }
}

//Peticion PATCH
pub fn handle_patch(request: &str, body: &str, db: &Database) -> String {
    let name_in_url = extract_name_from_request(request);

    let mut db = db.lock().unwrap();

    // Decodificar el nombre antes de buscarlo en la base de datos
    if let Some(person) = db.get_mut(&name_in_url) {
        let patch_data: serde_json::Value = match serde_json::from_str(body) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data.".to_string();
            }
        };

        // Actualizar solo los campos proporcionados
        if let Some(new_name) = patch_data.get("name").and_then(|v| v.as_str()) {
            person.name = new_name.to_string();
        }
        if let Some(new_age) = patch_data.get("age").and_then(|v| v.as_u64()) {
            person.age = new_age as u32;
        }
        if let Some(new_drinks) = patch_data.get("drinks").and_then(|v| v.as_array()) {
            person.drinks = new_drinks.iter().filter_map(|d| d.as_str().map(|s| s.to_string())).collect();
        }

        let person_json = serde_json::to_string(person).unwrap();
        format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", person_json)
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.".to_string()
    }
}

//Peticion DELETE
pub fn handle_delete(request: &str, db: &Database) -> String {
    let name_in_url = extract_name_from_request(request);

    let mut db = db.lock().unwrap();

    if db.contains_key(&name_in_url) {
        db.remove(&name_in_url);
        format!("HTTP/1.1 200 OK\r\n\r\nPerson {} deleted successfully.", name_in_url)
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.".to_string()
    }
}

// Pruebas
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_database() {
        let db = create_database();
        assert!(db.lock().unwrap().is_empty());
    }

    #[test]
    fn test_handle_post() {
        let db = create_database();
        let body = r#"{"name": "John Doe", "age": 30, "drinks": []}"#;
        let response = handle_post(body, &db);
        let expected_response = "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\"name\":\"John Doe\",\"age\":30,\"drinks\":[]}";
        assert_eq!(response, expected_response);
        assert!(db.lock().unwrap().contains_key("John Doe"));
    }

    #[test]
    fn test_handle_get_existing_person() {
        let db = create_database();
        let body = r#"{"name": "Jane Doe", "age": 25, "drinks": ["Water"]}"#;
        handle_post(body, &db); // Agregamos la persona primero

        let request = "GET /person/Jane%20Doe HTTP/1.1";
        let response = handle_get(request, &db);
        let expected_response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"name\":\"Jane Doe\",\"age\":25,\"drinks\":[\"Water\"]}";
        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_handle_get_non_existing_person() {
        let db = create_database();
        let request = "GET /person/NonExistent HTTP/1.1";
        let response = handle_get(request, &db);
        assert_eq!(response, "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.");
    }

    #[test]
    fn test_handle_put_existing_person() {
        let db = create_database();
        let body = r#"{"name": "Jane Doe", "age": 25, "drinks": ["Water"]}"#;
        handle_post(body, &db); // Agregamos la persona primero

        let request = "PUT /person/Jane%20Doe HTTP/1.1";
        let new_body = r#"{"name": "Jane Doe", "age": 26, "drinks": ["Water", "Tea"]}"#;
        let response = handle_put(request, new_body, &db);

        let expected_response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"name\":\"Jane Doe\",\"age\":26,\"drinks\":[\"Water\",\"Tea\"]}";
        assert_eq!(response, expected_response);
        assert!(db.lock().unwrap().contains_key("Jane Doe"));
    }

    #[test]
    fn test_handle_put_non_existing_person() {
        let db = create_database();
        let request = "PUT /person/NonExistent HTTP/1.1";
        let new_body = r#"{"name": "NonExistent", "age": 40, "drinks": ["Coffee"]}"#;
        let response = handle_put(request, new_body, &db);
        assert_eq!(response, "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.");
    }

    #[test]
    fn test_handle_patch_existing_person() {
        let db = create_database();
        let body = r#"{"name": "Jane Doe", "age": 25, "drinks": ["Water"]}"#;
        handle_post(body, &db); // Agregamos la persona primero

        let request = "PATCH /person/Jane%20Doe HTTP/1.1";
        let patch_body = r#"{"age": 27}"#; // Solo actualizamos la edad
        let response = handle_patch(request, patch_body, &db);

        let expected_response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"name\":\"Jane Doe\",\"age\":27,\"drinks\":[\"Water\"]}";
        assert_eq!(response, expected_response);
        assert_eq!(db.lock().unwrap().get("Jane Doe").unwrap().age, 27);
    }

    #[test]
    fn test_handle_patch_non_existing_person() {
        let db = create_database();
        let request = "PATCH /person/NonExistent HTTP/1.1";
        let patch_body = r#"{"age": 40}"#; // Intentamos actualizar una persona que no existe
        let response = handle_patch(request, patch_body, &db);
        assert_eq!(response, "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.");
    }

    #[test]
    fn test_handle_delete_existing_person() {
        let db = create_database();
        let body = r#"{"name": "Jane Doe", "age": 25, "drinks": ["Water"]}"#;
        handle_post(body, &db); // Agregamos la persona primero

        let request = "DELETE /person/Jane%20Doe HTTP/1.1";
        let response = handle_delete(request, &db);
        let expected_response = "HTTP/1.1 200 OK\r\n\r\nPerson Jane Doe deleted successfully.";
        assert_eq!(response, expected_response);
        assert!(!db.lock().unwrap().contains_key("Jane Doe")); // Verificamos que la persona fue eliminada
    }

    #[test]
    fn test_handle_delete_non_existing_person() {
        let db = create_database();
        let request = "DELETE /person/NonExistent HTTP/1.1";
        let response = handle_delete(request, &db);
        assert_eq!(response, "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.");
    }
}
