use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
    if db.contains_key(&new_person.name) {
        return "HTTP/1.1 409 Conflict\r\n\r\nPerson already exists.".to_string();
    }

    // Insertar la nueva persona en la base de datos
    db.insert(new_person.name.clone(), new_person.clone());

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
    // Suponemos que el nombre de la persona viene en la URL. Extrae el nombre del request.
    let lines: Vec<&str> = request.lines().collect();
    let url = lines[0].split_whitespace().nth(1).unwrap_or("");
    url.trim_start_matches("/person/").to_string()
}

//Peticion PUT
pub fn handle_put(request: &str, body: &str, db: &Database) -> String {
    // Extraer el nombre de la persona desde la URL
    let path = request.lines().next().unwrap_or("").split_whitespace().nth(1).unwrap_or("");
    let name_in_url = path.trim_start_matches("/person/");

    let updated_person: Person = match serde_json::from_str(body) {
        Ok(person) => person,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);  // Ver errores en la consola
            return "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data.".to_string();
        }
    };

    let mut db = db.lock().unwrap();

    // Verificar si la persona existe en la base de datos
    if db.contains_key(name_in_url) {
        // Si el nombre ha cambiado, eliminamos la entrada antigua y creamos una nueva con el nombre actualizado
        if updated_person.name != name_in_url {
            db.remove(name_in_url);  // Eliminamos la entrada antigua
        }

        // Insertamos o actualizamos con el nuevo nombre (en caso de que el nombre haya cambiado)
        db.insert(updated_person.name.clone(), updated_person.clone());

        let person_json = serde_json::to_string(&updated_person).unwrap();
        format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", person_json)
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.".to_string()
    }
}

//Peticion PATCH
pub fn handle_patch(request: &str, body: &str, db: &Database) -> String {
    // Extraer el nombre de la persona desde la URL
    let path = request.lines().next().unwrap_or("").split_whitespace().nth(1).unwrap_or("");
    let name_in_url = path.trim_start_matches("/person/");

    let mut db = db.lock().unwrap();

    // Verificar si la persona existe en la base de datos
    if let Some(person) = db.get_mut(name_in_url) {
        // Intenta deserializar el JSON en un mapa de claves y valores
        let patch_data: serde_json::Value = match serde_json::from_str(body) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data.".to_string();
            }
        };

        //println!("Patch data: {:?}", patch_data);  // Depuración

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

        // Devolver la persona actualizada
        let person_json = serde_json::to_string(person).unwrap();
        format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", person_json)
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.".to_string()
    }
}


//Peticion DELETE
pub fn handle_delete(request: &str, db: &Database) -> String {
    // Extraer el nombre de la persona desde la URL
    let path = request.lines().next().unwrap_or("").split_whitespace().nth(1).unwrap_or("");
    let name_in_url = path.trim_start_matches("/person/");

    let mut db = db.lock().unwrap();

    // Verificar si la persona existe en la base de datos
    if db.contains_key(name_in_url) {
        // Eliminar la entrada de la base de datos
        db.remove(name_in_url);

        format!("HTTP/1.1 200 OK\r\n\r\nPerson {} deleted successfully.", name_in_url)
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nPerson not found.".to_string()
    }
}