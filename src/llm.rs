use config::Config;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

#[derive(Deserialize)]
struct OllamaConfig {
    model: String,
    url: String,
}

async fn get_ollama_config() -> Result<OllamaConfig, Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    let ollama_config: OllamaConfig = settings.get("ollama")?;
    Ok(ollama_config)
}

pub async fn chat_with_llm(user_message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ollama_config = get_ollama_config().await?;

    let context = "Eres un bot de Discord amigable y útil. Tu objetivo es ayudar a los usuarios con sus preguntas y mantener conversaciones divertidas e informativas. Responde de manera concisa y clara. Ahora, este es el mensaje del usuario: \n";

    let client = Client::new();
    let request_body = OllamaRequest {
        model: ollama_config.model,
        prompt: context.to_owned() + user_message,
        stream: false,
    };

    let response = client
        .post(&ollama_config.url)
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let response_json: OllamaResponse = response.json().await?;
        Ok(response_json.response)
    } else {
        let error_message = format!(
            "Fallo al obtener respuesta de Ollama: {}",
            response.status()
        );
        error!("{}", error_message);
        Err(error_message.into())
    }
}
