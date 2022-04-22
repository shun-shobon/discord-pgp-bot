use std::env;

use anyhow::Result;
use serenity::http::Http;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;

#[tokio::main]
async fn main() -> Result<()> {
    let discord_token = env::var("DISCORD_TOKEN")?;
    let application_id = env::var("DISCORD_APPLICATION_ID")?;
    let guild_id = env::var("DISCORD_GUILD_ID")?;
    let http = Http::new_with_application_id(&discord_token, application_id.parse()?);
    let commands = GuildId(guild_id.parse()?)
        .set_application_commands(http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("pgp")
                    .description("PGP鍵関連の処理をします")
                    .create_option(|option| {
                        option
                            .name("register")
                            .description("PGP鍵を登録します")
                            .kind(ApplicationCommandOptionType::SubCommand)
                            .create_sub_option(|option| {
                                option
                                    .name("url")
                                    .description("PGP鍵が存在するURL")
                                    .kind(ApplicationCommandOptionType::String)
                                    .required(true)
                            })
                    })
                    .create_option(|option| {
                        option
                            .name("help")
                            .description("ヘルプを表示します")
                            .kind(ApplicationCommandOptionType::SubCommand)
                    })
            })
        })
        .await?;

    println!("Success to register application commands: {:#?}", commands);

    Ok(())
}
