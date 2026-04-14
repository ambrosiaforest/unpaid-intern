use poise::serenity_prelude as serenity;

use rand;

use dotenv;

mod db;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn balance(
    ctx: Context<'_>
) -> Result<(), Error> {
    let user = ctx.author().id.to_string();

    let balance = match db::get_balance(&user) {
        Ok(Some(b)) => b,
        Ok(None) => { let _ = db::insert_user(&user, 2000); 2000 },
        Err(_) => 0,
    };

    ctx.say(format!("Balance: ${}", balance)).await?;
    
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn cointoss(
    ctx: Context<'_>, 
    #[description = "Bet"] bet: i32,
    #[description = "Call"] call: String
) -> Result<(), Error> {
    let flip = rand::random_range(0..2);
    let user = ctx.author().id.to_string();
    let calln = match call.to_lowercase().as_str() {
        "heads" => 1,
        "tails" => 0,
        _ => -1
    };

    let balance = match db::get_balance(&user) {
        Ok(Some(b)) => b,
        Ok(None) => { let _ = db::insert_user(&user, 2000); 2000 },
        Err(_) => 0,
    };

    let mut winnings = 0;

    if balance < bet {
        ctx.say("Insufficent funds").await?;
        return Ok(());
    }

    let reply = if calln == flip {
        winnings += bet * 2;
        poise::CreateReply::default()
            .content(format!("{} bet ${} on {}", ctx.author(), bet, call))
            .embed(serenity::CreateEmbed::new()
                .title("——Result——")
                .description(format!("Won: ${}", winnings))
            )
    } else if calln == -1 {
        poise::CreateReply::default()
            .embed(serenity::CreateEmbed::new()
                .title("❌Invalid❌")
                .description("Call must be heads or tails")
            )
            .ephemeral(true)
    } else {
        winnings -= bet;
        poise::CreateReply::default()
            .content(format!("{} bet ${} on {}", ctx.author(), bet, call))
            .embed(serenity::CreateEmbed::new()
                .title("—Result—")
                .description("Lost it all")
            )
    };

    let _ = db::set_balance(&user, balance + winnings)?;

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

    let http = ctx.serenity_context().http.clone();
    let channel_id = ctx.channel_id();

    let builder = serenity::CreateMessage::new().content(text);
    let _ = channel_id.send_message(&http, builder).await;

    ctx.send(poise::CreateReply::default()
        .content("Message sent successfully!")
        .ephemeral(true)
    ).await?;

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
                balance(),
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
    let _ = db::create_database();

    //db::insert_user("123456789", 500);
    //println!("Users in database:");
    let _ = db::query_users();

    if let Ok(Some(balance)) = db::get_balance("123456789") {
        println!("Balance: {}", balance);
    } else {
        println!("User not found");
    }

    let activity = serenity::ActivityData::custom("jorkin it");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .activity(activity)
        .await;

    client.unwrap().start().await.unwrap();
}
