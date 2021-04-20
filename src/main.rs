#![allow(non_snake_case)]
extern crate rand;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use rand::thread_rng;
use rand::Rng;

const ECHO_COMMAND: &str = "!echo";
const HELP_COMMAND: &str = "!help";
const DICE_ROLL: &str = "!roll";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx: Context, msg: Message) {
        let usr_mess: Vec<&str> = msg.content.split_whitespace().collect();
        if usr_mess[0] == ECHO_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, usr_mess[1]).await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if usr_mess[0] == HELP_COMMAND
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "You're on your own rn my friend.").await {
                println!("Error sending message: {:?}", why);
            } 
        }
        else if usr_mess[0] == DICE_ROLL // ex: !roll 4 6 where its 4 six-sided dice
        {
            //gets both numbers from the message and converts them to unsigned 32 bit integers
            //might only need u8 here??

            let numRolls: u32 = usr_mess[1].parse::<u32>().unwrap();
            let sides: u32 = usr_mess[2].parse::<u32>().unwrap();
            let mut sum = 0;
            for _x in 0..numRolls
            { 
                let roll = dice_roll(sides);
                if let Err(why) = msg.channel_id.say(&ctx.http, roll).await {
                    println!("Error sending message: {:?}", why);
                } 
                sum += roll;
            }
            let tempmsg = "Roll sum ".to_owned() + &sum.to_string();
            if let Err(why) = msg.channel_id.say(&ctx.http, tempmsg).await {
                println!("Error sending message: {:?}", why);
            } 
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

//simulates rolling of a dice
// specify how many dice and how many sides
// currently only supports several dice of the same type
fn dice_roll(sides: u32) -> u32
    {
        let mut rng = thread_rng();
        let y: u32 = rng.gen_range(1,sides);
        return y;
    }

#[tokio::main]
async fn main() {
    //set an environmental variable in the shell to run this currently
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}