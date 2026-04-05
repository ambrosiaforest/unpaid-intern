use poise::serenity_prelude as serenity;
use rand::prelude::*;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn say(
    ctx: Context<'_>,
    #[description = "text to say"] text: String,
) -> Result<(), Error> {
    let response = format!("{}", text);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn jorkit(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response: String = match rand::random_range(0..4) {
        0 => "https://tenor.com/hu6bIuxX2f4.gif".to_string(),
        1 => "https://cdn.discordapp.com/emojis/1485862202224283759.webp?size=240&animated=true".to_string(),
        2 => "https://cdn.discordapp.com/attachments/1336182573214334986/1419170851311587488/IMG_2838.gif".to_string(),
        3 => "https://media.discordapp.net/attachments/1343399089412374539/1475040324358373547/NNvEX54tL51dCeICkPb1nuUhqCtJashBkeZYS_sUxdA.gif".to_string(),
        _ => "faggot".to_string()
    };

    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), say(), jorkit()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
