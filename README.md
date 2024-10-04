# Proyecto #1: Servidor HTTP Implementado en Rust
## Tecnológico de Costa Rica II Semestre 2024
### Estudiantes: Tomás Granados Preciado, Jose Pablo Granados Siles, Stiven Segura Guzmán

## Descripción General
Este proyecto implementa un servidor HTTP básico utilizando el lenguaje de programación Rust. El servidor soporta las operaciones HTTP más comunes como `GET`, `POST`, `PUT`, `DELETE` y `UPDATE`, y maneja múltiples solicitudes de forma concurrente utilizando un pool de hilos. Además, el servidor gestiona cookies de sesión para mantener un estado básico entre solicitudes.

## Arquitectura del Servidor
El servidor está construido sobre la biblioteca estándar de Rust y se organiza de manera modular, dividiendo las responsabilidades en diferentes archivos (`main.rs`, `cookies.rs`, `thread_pool.rs`, y `person.rs`). La arquitectura sigue el siguiente flujo básico:

1. **Manejo de solicitudes HTTP:** El servidor escucha en un puerto específico y acepta conexiones entrantes. Cada conexión se procesa en un hilo separado para asegurar la concurrencia.
2. **Operaciones HTTP soportadas:**
   - `GET`: Recupera recursos almacenados en el servidor.
   - `POST`: Envía datos para su procesamiento o almacenamiento.
   - `PUT`: Crea o actualiza recursos en el servidor.
   - `DELETE`: Elimina recursos específicos.
   - `UPDATE`: Modifica parcialmente un recurso existente.
3. **Concurrencia:** Utilizamos un pool de hilos para manejar múltiples solicitudes concurrentes. El número de hilos puede configurarse según las necesidades del servidor para optimizar el rendimiento.
4. **Cookies de sesión:** Se implementa un manejo básico de cookies para mantener el estado de la sesión de los usuarios.

## Concurrencia
El servidor utiliza un **pool de hilos** para procesar múltiples conexiones simultáneamente. Esto se implementa en el archivo `thread_pool.rs`, donde se crea un conjunto de hilos fijos que toman tareas de una cola. Cuando llega una nueva solicitud, se añade a la cola y un hilo disponible la procesa.

### Implementación del pool de hilos:
```rust
// thread_pool.rs
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
```
El pool de hilos asegura que el servidor no se bloquee al manejar múltiples conexiones concurrentemente y mitiga las condiciones de carrera mediante el uso de mutexes y canales.

### Manejo de Cookies

El manejo de cookies se realiza en el archivo `cookies.rs`. Este módulo permite crear, almacenar y eliminar cookies de sesión que mantienen el estado de un usuario entre las diferentes solicitudes.

### Ejemplo

```rust
// cookies.rs
pub struct Cookies {
    pub cookies: HashMap<String, String>,
}

impl Cookies {
    pub fn new() -> Cookies {
        Cookies {
            cookies: HashMap::new(),
        }
    }

    pub fn set_cookie(&mut self, key: String, value: String) {
        self.cookies.insert(key, value);
    }

    pub fn get_cookie(&self, key: &str) -> Option<&String> {
        self.cookies.get(key)
    }

    pub fn delete_cookie(&mut self, key: &str) {
        self.cookies.remove(key);
    }
}
```
Cada usuario tiene una cookie de sesión única que se gestiona con un mapa hash. Estas cookies permiten mantener el estado de inicio de sesión o cualquier otra información entre las solicitudes.

## Instrucciones para Ejecutar el Servidor
Para ejecutar el servidor, sigue los siguientes pasos:

1. Clone el repositorio o descarga los archivos del proyecto.
2. Asegúrese de tener Rust instalado en tu sistema. Si no lo tienes, instálalo siguiendo las instrucciones en [Rust Installation Guide](https://www.rust-lang.org/tools/install).
3. Compile el proyecto utilizando el comando:
   ```bash
   cargo build
   ```
4. Ejecute el servidor con el comando:
   ```bash
   cargo run
   ```
   El servidor escuchará en el puerto especificado en el código.

## API de Gestión de Personas
### 1. *Crear una Persona (POST)*

*Endpoint:*  
POST /person

*Cuerpo de la solicitud (Body):*
json
{
    "name": "Joe",
    "age": 21
}


- *Campos obligatorios:*
  - name (string): Nombre de la persona.
  - age (integer): Edad de la persona.
  
- *Campo opcional:*
  - drinks (array de strings): Lista de bebidas favoritas de la persona. Si no se especifica, se creará con un array vacío.

*Descripción:*  
Este endpoint crea un nuevo recurso persona. Si no se especifica la lista de drinks, se asignará un arreglo vacío por defecto.

*Respuesta exitosa:*  
- Código: 201 Created
- Cuerpo:
  json
  {
      "name": "Joe",
      "age": 21,
      "drinks": []
  }
  

---

### 2. *Obtener una Persona (GET)*

*Endpoint:*  
GET /person/{name}

*Parámetros de ruta:*
- name (string): Nombre de la persona que se desea obtener.

*Descripción:*  
Este endpoint devuelve la información de una persona especificada por su name.

*Ejemplo de solicitud:*  
GET /person/Joe

*Respuesta exitosa:*  
- Código: 200 OK
- Cuerpo:
  json
  {
      "name": "Joe",
      "age": 21,
      "drinks": []
  }

### 3. *Actualizar Parcialmente una Persona (PATCH)*

*Endpoint:*  
PATCH /person/{name}

*Parámetros de ruta:*
- name (string): Nombre de la persona que se desea actualizar.

*Cuerpo de la solicitud (Body):*
json
{
    "drinks": ["Tequila"]
}


*Descripción:*  
Este endpoint permite actualizar parcialmente un recurso existente. Solo las claves enviadas en el cuerpo de la solicitud serán actualizadas. Por ejemplo, se puede añadir o modificar la lista de drinks sin afectar los demás campos.

*Ejemplo de solicitud:*  
PATCH /person/Joe

*Respuesta exitosa:*  
- Código: 200 OK
- Cuerpo:
  json
  {
      "name": "Joe",
      "age": 21,
      "drinks": ["Tequila"]
  }
  

---

### 4. *Actualizar Completamente una Persona (PUT)*

*Endpoint:*  
PUT /person/{name}

*Parámetros de ruta:*
- name (string): Nombre de la persona que se desea actualizar.

*Cuerpo de la solicitud (Body):*
json
{
    "name": "Joe",
    "age": 28
}


*Descripción:*  
Este endpoint reemplaza completamente los datos de un recurso existente. Si algún campo no es especificado en el cuerpo de la solicitud, será eliminado. En este caso, drinks no se especifica, por lo que el campo será eliminado o reemplazado con un valor vacío.

*Ejemplo de solicitud:*  
PUT /person/Joe

*Respuesta exitosa:*  
- Código: 200 OK
- Cuerpo:
  json
  {
      "name": "Joe",
      "age": 28,
      "drinks": []
  }
  

---

### 5. *Eliminar una Persona (DELETE)*

*Endpoint:*  
DELETE /person/{name}

*Parámetros de ruta:*
- name (string): Nombre de la persona que se desea eliminar.

*Descripción:*  
Este endpoint elimina el recurso de una persona especificada por su name.

*Ejemplo de solicitud:*  
DELETE /person/Joe

*Respuesta exitosa:*  
- Código: 204 No Content

---

### Códigos de Estado Comunes:
- *200 OK:* Solicitud exitosa.
- *201 Created:* Recurso creado exitosamente.
- *204 No Content:* Recurso eliminado o no hay contenido para devolver.
- *400 Bad Request:* Solicitud malformada.
- *404 Not Found:* Recurso no encontrado.

## Estructura de Directorios
El proyecto está organizado de la siguiente manera:

```
.
├── src/
│   ├── main.rs         # Punto de entrada principal del servidor
│   ├── cookies.rs      # Módulo de manejo de cookies
│   ├── thread_pool.rs  # Módulo para el pool de hilos
│   └── person.rs       # Módulo de manejo de recursos (personas)
├── Cargo.toml          # Archivo de configuración de Rust
└── README.md           # Documentación del proyecto
```

## Pruebas Unitarias
Las pruebas unitarias cubren las principales funcionalidades del servidor, incluyendo las operaciones HTTP y el manejo de cookies. Asegúrate de correr los tests con el comando:
```bash
cargo test
```

## Capturas de Pruebas
A continuación, capturas de las pruebas realizadas con `postman` sobre pruebas de casos exitosos, tanto como los tipos de errores. 

### Pruebas de Casos exitosos

[Click para ver imagen](https://drive.google.com/file/d/14ctQJtlRcVLjbO9ieoWuJ6TJajRrMfiQ/view?usp=sharing)

### Pruebas de Errores

[Click para ver imagen](https://drive.google.com/file/d/1c0AaFGHCnLjQCTwamWIP8SuZZzcFjAQf/view?usp=sharing)















