use poise::serenity_prelude as serenity;

use rand;

use dotenv;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn cointoss(
    ctx: Context<'_>, 
    #[description = "Bet"] bet: u32,
    #[description = "Call"] call: String
) -> Result<(), Error> {
    let flip = rand::random_range(0..2);

    let calln = match call.to_lowercase().as_str() {
        "heads" => 1,
        "tails" => 0,
        _ => -1
    };

    let reply = if calln == flip {
        poise::CreateReply::default()
            .content(format!("{} bet ${} on {}", ctx.author(), bet, call))
            .embed(serenity::CreateEmbed::new()
                .title("——Result——")
                .description(format!("Won: ${}", bet*2))
            )
    } else if calln == -1 {
        poise::CreateReply::default()
            .embed(serenity::CreateEmbed::new()
                .title("❌Invalid❌")
                .description("Call must be heads or tails")
            )
            .ephemeral(true)
    } else {
        poise::CreateReply::default()
            .content(format!("{} bet ${} on {}", ctx.author(), bet, call))
            .embed(serenity::CreateEmbed::new()
                .title("—Result—")
                .description("Lost it all")
            )
    };

    ctx.send(reply).await?;
    Ok(())
}



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

    ctx.say(text).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn jorkit(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response: String = match rand::random_range(0..5) {
        0 => "https://tenor.com/view/invincible-variant-tracksuit-mark-invincible-edit-invincible-gif-16907687914240760559".to_string(),
        1 => "https://cdn.discordapp.com/emojis/1485862202224283759.webp?size=240&animated=true".to_string(),
        2 => "https://cdn.discordapp.com/attachments/1336182573214334986/1419170851311587488/IMG_2838.gif".to_string(),
        3 => "https://media.discordapp.net/attachments/1343399089412374539/1475040324358373547/NNvEX54tL51dCeICkPb1nuUhqCtJashBkeZYS_sUxdA.gif".to_string(),
		4 => "https://giphy.com/gifs/invincible-ichi39-variants-MPDOGLMCXtQNdQR7SE".to_string(),
        _ => "faggot".to_string()
    };

    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(),
                say(),
                jorkit(),
                cointoss()
            ],
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
