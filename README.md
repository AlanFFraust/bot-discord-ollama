# Bot de Discord con Ollama

Este proyecto es un bot de Discord desarrollado en Rust que integra un modelo de lenguaje utilizando Ollama para correr los modelos de forma local. El bot responde a comandos enviados en Discord, permitiendo saludar, mostrar ayuda y mantener conversaciones utilizando un modelo de lenguaje.

## Características

- **Interacción en Discord:** Responde a comandos específicos enviados por los usuarios.
- **Integración con Ollama:** Utiliza un modelo de lenguaje local para generar respuestas en conversaciones.
- **Comandos disponibles:**
  - `!bot hello` – El bot te saluda.
  - `!bot help` – Muestra un mensaje de ayuda con los comandos disponibles.
  - `!bot chat <mensaje>` – Envía el mensaje al modelo de lenguaje y devuelve una respuesta generada.

## Requisitos

- **Rust:** Asegúrate de tener instalado Rust. Puedes instalarlo desde [rustup.rs](https://rustup.rs/).
- **Ollama:** Debes tener Ollama instalado y en ejecución para usar los modelos de lenguaje localmente. Consulta la [documentación de Ollama](https://ollama.com/) para más información.

## Configuración

### Archivo `.env`

Crea un archivo `.env` en la raíz del proyecto para almacenar el token de tu bot de Discord. Por ejemplo:

```dotenv
DISCORD_TOKEN=tu_token_de_discord_aqui
```


Reemplaza `tu_token_de_discord_aqui` con el token real de tu bot.

### Archivo `config.toml`

El archivo `config.toml` se utiliza para configurar la conexión con Ollama. Modifica este archivo en la raíz del proyecto para utilizar otros modelos de lenguaje o URLs diferentes. Por ejemplo:

```
[ollama]
model = "deepseek-r1:7b"
url = "http://localhost:11434/api/generate"
```

-   **model:** Especifica el modelo de lenguaje que deseas utilizar.
    
-   **url:** La URL del endpoint de Ollama para generar respuestas.  
    Asegúrate de que Ollama esté corriendo y que la URL sea correcta.
    

## Instalación y Ejecución

-   **Clona el repositorio:**
    
    ```
    git clone https://github.com/AlanFFraust/bot-discord-ollama
    cd bot-discord-ollama
    ```
    
-   **Compila el proyecto:**
    
    ```
    cargo build --release
    ```
    
-   **Ejecuta el bot:**
    
    ```
    cargo run --release
    ```
    

El bot se conectará a Discord utilizando el token proporcionado y comenzará a escuchar mensajes en los canales configurados.

## Uso

Una vez en funcionamiento, puedes interactuar con el bot en Discord mediante los siguientes comandos:

-   `!bot hello`  
    El bot te responderá con un saludo.
    
-   `!bot help`  
    Muestra un mensaje con la lista de comandos disponibles.
    
-   `!bot chat <mensaje>`  
    Envía el `<mensaje>` al modelo de lenguaje de Ollama y devuelve la respuesta generada.
    

## Contribuciones

Las contribuciones son bienvenidas. Si encuentras errores o tienes ideas para mejorar el bot, por favor abre un issue o envía un pull request.

## Licencia

Este proyecto se distribuye bajo la Licencia MIT.
