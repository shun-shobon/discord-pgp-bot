mod database;

use crate::database::UserRepository;
use async_trait::async_trait;
use database::mem::MemUserRepository;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::{GatewayIntents, TypeMapKey};
use serenity::utils::MessageBuilder;
use serenity::Client;
use std::env;

struct UserRepositoryData;

impl TypeMapKey for UserRepositoryData {
    type Value = Box<dyn UserRepository + Send + Sync>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // TODO: あまりにも汚いのでどうにかする
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "pgp" => {
                    let subcommand = command.data.options.first().unwrap();
                    let options = &subcommand.options;
                    let subcommand = subcommand.name.as_ref();

                    match subcommand {
                        "register" => {
                            let url = options
                                .iter()
                                .find(|option| option.name == "url")
                                .unwrap()
                                .to_owned()
                                .resolved
                                .unwrap();

                            let user_repository =
                                ctx.data.write().await.get::<UserRepositoryData>().unwrap();

                            if let ApplicationCommandInteractionDataOptionValue::String(url) = url {
                                MessageBuilder::new()
                                    .push_mono(url)
                                    .push("を登録しました")
                                    .build()
                            } else {
                                unreachable!()
                            }
                        }
                        "help" => include_str!("./help.txt").to_owned(),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };

            if let Err(err) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                eprintln!("Cannot responed to slash command: {}", err);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_TOKEN")?;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    client
        .data
        .write()
        .await
        .insert::<UserRepositoryData>(Box::new(MemUserRepository::new()));

    client.start().await?;

    Ok(())
}
