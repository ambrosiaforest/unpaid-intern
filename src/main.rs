use dotenv;
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use rand;
use serenity::CreateEmbed;
use std::time::Instant;
use unpaid_intern::{create_database, get_balance, insert_user, query_users, set_balance};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn balance(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author().id.to_string();
    let name = ctx.author().name.to_string();

    let http = ctx.serenity_context().http.clone();

    let balance = match get_balance(&user) {
        Ok(Some(b)) => b,
        Ok(None) => {
            let _ = insert_user(&user, 2000);
            2000
        }
        Err(_) => 0,
    };

    let title = format!("{name}'s Balance");
    let description = format!("${balance}");

    let embed = CreateEmbed::new().title(title).description(description);

    let builder = CreateReply::default().embed(embed);

    ctx.send(builder).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn cointoss(
    ctx: Context<'_>,
    #[description = "Bet"] bet: i32,
    #[description = "Call"] call: String,
) -> Result<(), Error> {
    let flip = rand::random_range(0..2);
    let user = ctx.author().id.to_string();

    let name = ctx.author().name.to_string();

    let calln = match call.to_lowercase().as_str() {
        "heads" => 1,
        "tails" => 0,
        _ => -1,
    };

    let balance = match get_balance(&user) {
        Ok(Some(b)) => b,
        Ok(None) => {
            let _ = insert_user(&user, 2000);
            2000
        }
        Err(_) => 0,
    };

    let mut winnings = 0;

    if balance < bet {
        ctx.say("Insufficent funds").await?;
        return Ok(());
    }

    let mut title = String::new();
    let mut description = String::new();

    if calln == flip {
        winnings += bet;
        title = format!("{} bet ${bet} on {call}", name);
        description = format!("### —Result—\nWon: ${}", winnings);
    } else if calln == -1 {
        title = format!("{} bet ${} on {}", name, bet, call);
        description = "### ❌Invalid❌\nCall must be heads or tails".to_string();
    } else {
        winnings -= bet;
        title = format!("{} bet ${} on {}", name, bet, call);
        description = "### —Result—\nLost it all".to_string();
    };

    let embed = CreateEmbed::new().title(title).description(description);
    let builder = CreateReply::default().embed(embed);

    let _ = set_balance(&user, balance + winnings)?;

    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();

    ctx.defer().await?;

    let latency_ms = start.elapsed().as_millis();

    ctx.say(format!("Pong! Bot response time: {} ms", latency_ms))
        .await?;

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
async fn say(ctx: Context<'_>, #[description = "text to say"] text: String) -> Result<(), Error> {
    let http = ctx.serenity_context().http.clone();
    let channel_id = ctx.channel_id();

    let builder = serenity::CreateMessage::new().content(text);
    let _ = channel_id.send_message(&http, builder).await;

    ctx.send(
        poise::CreateReply::default()
            .content("Message sent successfully!")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn jorkit(ctx: Context<'_>) -> Result<(), Error> {
    let response: String = match rand::random_range(0..5) {
        0 => "https://tenor.com/view/invincible-variant-tracksuit-mark-invincible-edit-invincible-gif-16907687914240760559".to_string(),
        1 => "https://cdn.discordapp.com/emojis/1485862202224283759.webp?size=240&animated=true".to_string(),
        2 => "https://cdn.discordapp.com/attachments/1336182573214334986/1419170851311587488/IMG_2838.gif".to_string(),
        3 => "https://media.discordapp.net/attachments/1343399089412374539/1475040324358373547/NNvEX54tL51dCeICkPb1nuUhqCtJashBkeZYS_sUxdA.gif".to_string(),
		4 => "https://giphy.com/gifs/invincible-ichi39-variants-MPDOGLMCXtQNdQR7SE".to_string(),
        _ => String::new(),
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
            commands: vec![age(), ping(), say(), jorkit(), balance(), cointoss()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let _ = create_database();

    let _ = query_users();

    let activity = serenity::ActivityData::custom("jorkin it");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .activity(activity)
        .await;

    client.unwrap().start().await.unwrap();
}
