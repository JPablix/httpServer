---

# *Manual de Usuario: Servidor HTTP Gestión de Personas*

### Bienvenido
Este manual está diseñado para guiarle en el uso de la API, permitiéndole crear, consultar, actualizar y eliminar personas de manera sencilla. No es necesario ser un experto, solo siga los pasos indicados a continuación.

---

## 1. *Cómo Crear una Persona (POST)*

*¿Qué hace esto?*  
Este método le permite añadir una nueva persona a la API.

*Pasos a seguir:*

1. Abra la herramienta de su preferencia (Postman, cURL, etc.).
2. Seleccione el método POST.
3. En la barra de dirección, escriba:  
   http://localhost:8080/person
4. En la sección de "Body" (cuerpo de la solicitud), introduzca los siguientes datos mínimos:
   json
   {
       "name": "Joe",
       "age": 21
   }
   
   - Solo el *nombre* y la *edad* son obligatorios.
   - Si no incluye las bebidas, no se preocupe, la API lo hará por usted, asignando una lista vacía.

5. Haga clic en "Enviar" (Send).

*¿Qué sucede después?*  
Se creará una nueva persona en la API, y recibirá una respuesta mostrando la persona creada:
json
{
    "name": "Joe",
    "age": 21,
    "drinks": []
}


---

## 2. *Cómo Buscar una Persona (GET)*

*¿Qué hace esto?*  
Este método le permite buscar la información de una persona en la API utilizando su nombre.

*Pasos a seguir:*

1. Seleccione el método GET.
2. En la barra de dirección, escriba:  
   http://localhost:8080/person/Joe  
   (Reemplace "Joe" por el nombre de la persona que desea buscar).
3. Haga clic en "Enviar" (Send).

*¿Qué sucede después?*  
Recibirá toda la información de la persona, incluyendo nombre, edad y bebidas favoritas:
json
{
    "name": "Joe",
    "age": 21,
    "drinks": []
}


---

## 3. *Cómo Actualizar una Parte de la Información (PATCH)*

*¿Qué hace esto?*  
Este método le permite modificar solo una parte de la información de la persona, por ejemplo, añadir una bebida favorita.

*Pasos a seguir:*

1. Seleccione el método PATCH.
2. En la barra de dirección, escriba:  
   http://localhost:8080/person/Joe  
   (Reemplace "Joe" por el nombre de la persona).
3. En la sección de "Body", introduzca los datos que desea actualizar. Ejemplo para agregar una bebida:
   json
   {
       "drinks": ["Tequila"]
   }
   
4. Haga clic en "Enviar" (Send).

*¿Qué sucede después?*  
La API actualizará solo la información enviada (en este caso, las bebidas). La respuesta será algo similar a:
json
{
    "name": "Joe",
    "age": 21,
    "drinks": ["Tequila"]
}


---

## 4. *Cómo Reemplazar Toda la Información (PUT)*

*¿Qué hace esto?*  
Este método le permite reemplazar completamente la información de una persona. Si no incluye algún campo, como las bebidas, este será eliminado.

*Pasos a seguir:*

1. Seleccione el método PUT.
2. En la barra de dirección, escriba:  
   http://localhost:8080/person/Joe
3. En la sección de "Body", ingrese la nueva información completa. Ejemplo:
   json
   {
       "name": "Joe",
       "age": 28
   }
   
4. Haga clic en "Enviar" (Send).

*¿Qué sucede después?*  
Toda la información de la persona será reemplazada con los nuevos datos proporcionados. Si algún dato no es especificado, como las bebidas, será eliminado. La respuesta será algo como:
json
{
    "name": "Joe",
    "age": 28,
    "drinks": []
}


---

## 5. *Cómo Eliminar una Persona (DELETE)*

*¿Qué hace esto?*  
Este método le permite eliminar a una persona de la API.

*Pasos a seguir:*

1. Seleccione el método DELETE.
2. En la barra de dirección, escriba:  
   http://localhost:8080/person/Joe  
   (Reemplace "Joe" por el nombre de la persona que desea eliminar).
3. Haga clic en "Enviar" (Send).

*¿Qué sucede después?*  
La persona será eliminada de la API, y no recibirá ningún contenido como respuesta, pero la operación habrá sido exitosa.

---

### Consejos adicionales:
- Asegúrese de que el servidor esté en funcionamiento antes de enviar solicitudes.
- Si recibe un error "404 Not Found", significa que la persona que está buscando no existe.
- Si recibe un error "400 Bad Request", asegúrese de que los datos enviados son correctos y completos.

---

Con estos sencillos pasos, podrá gestionar personas en la API sin dificultad.