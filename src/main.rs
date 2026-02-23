use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "$ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "faggot").await {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.contains("$say") {
            if let Err(why) = msg.channel_id.say(&ctx.http, msg.content.to_string().replace("$say", "")).await {
                println!("Error sending message: {why:?}");
            }
        }
    }
/*
    async fn ready(&self, ctx: Context, ready: Ready) {
       info!("{} is connected!", ready.user.name);
       
       let guild_id = GuildId();
       
       // add "/hello" command to the bot
       GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
           commands.create_application_command(|command| { command.name("hello").description("Say hello") })
       }).await.unwrap();
    } */
/*
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
       // check if the interaction is a command
       if let Interaction::ApplicationCommand(command) = interaction {

           let response_content =
               match command.data.name.as_str() {
                   "hello" => "hello".to_owned(),
                   command => unreachable!("Unknown command: {}", command),
               };
           // send `response_content` to the discord server
           command.create_interaction_response(&ctx.http, |response| {
               response
                   .kind(InteractionResponseType::ChannelMessageWithSource)
                   .interaction_response_data(|message| message.content(response_content))
           })
               .await.expect("Cannot respond to slash command");
       }
   } */
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
