use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

pub mod llm;

struct Bot;

impl Bot {
    fn new() -> Self {
        Self
    }

    async fn handle_command(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = msg.content.to_lowercase();

        let args: Vec<&str> = content.split_whitespace().collect();
        let command = args.get(1).unwrap_or(&"");

        let response = match *command {
            "hello" => "¡Hola! ¿En qué puedo ayudarte hoy?".to_string(),
            "help" => "Estos son los comandos que soporto:\n- `!bot hello`: ¡Salúdame!\n- `!bot help`: Muestra este mensaje de ayuda.\n- `!bot chat <mensaje>`: ¡Chatea conmigo usando el modelo de lenguaje!".to_string(),
            "chat" => {
                let chat_message = msg.content.trim_start_matches("!bot chat ").trim();
                match llm::chat_with_llm(chat_message).await {
                    Ok(res) => res,
                    Err(e) => {
                        error!("Error generando respuesta del modelo de lenguaje: {:?}", e);
                        format!("Lo siento, ocurrió un error: {}", e)
                    }
                }
            }
            _ => "Comando desconocido. Usa `!bot help` para ver los comandos disponibles.".to_string(),
        };

        if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
            error!("Error enviando mensaje: {:?}", e);
        }
        Ok(())
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().starts_with("!bot") {
            if let Err(e) = self.handle_command(&ctx, &msg).await {
                error!("Error al manejar el comando: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} está conectado!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("'DISCORD_TOKEN' no encontrado");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(Bot::new())
        .await
        .expect("Error al crear el cliente");

    if let Err(e) = client.start().await {
        error!("Error en el cliente: {:?}", e);
    }
}
