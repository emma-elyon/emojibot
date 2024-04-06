use std::fs;
// use std::io;

use serenity::all::CreateAttachment;
use unicode_segmentation::UnicodeSegmentation;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

const GSTATIC_PREFIX: &str = "https://www.gstatic.com/android/keyboard/emojikitchen/";
const GSTATIC_REVISIONS: [&str; 27] = [
	"20231113",
	"20201001",
	"20230418",
	"20230803",
	"20211115",
	"20230301",
	"20220815",
	"20230127",
	"20220203",
	"20221101",
	"20210831",
	"20220506",
	"20220406",
	"20210218",
	"20230126",
	"20231128",
	"20230821",
	"20230216",
	"20220110",
	"20221107",
	"20210521",
	"20230818",
	"20230426",
	"20230421",
	"20220823",
	"20230221",
	"20230613"
];

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content == "!ping" {
			if let Err(error) = msg.channel_id.say(&ctx.http, "pong!").await {
				eprintln!("Error sending message: {error:?}");
			}
		} else {
			let code_points = msg
				.content
				.replace(" ", "")
				.graphemes(true).collect::<Vec<&str>>()
				.iter()
				.map(|s| s
					.chars()
					.filter(|c| *c != ' ')
					.map(|c| c
						.escape_unicode()
						.to_string()
						.chars()
						.skip(3)
						.collect::<String>()
					)
					.map(|s| format!("u{}", &s[..s.len() - 1]))
					.collect::<Vec<_>>()
					.join("-")
				)
				.collect::<Vec<_>>();

			if let [first, last] = &code_points[..] {
				let guild_id = msg.guild_id.unwrap();
				for revision in GSTATIC_REVISIONS {
					let target = format!("{GSTATIC_PREFIX}{revision}/{first}/{first}_{last}.png");
					let response = reqwest::get(target.clone()).await.expect("Could not fetch file.");
					let status = response.status();
					if status.is_success() {
						let content = response.bytes().await.expect("Could not get bytes from response.");
						let guild_id = msg.guild_id.unwrap();
						let emoji_name = format!("{first}_{last}").replace("-", "_");
						let image = CreateAttachment::bytes(content, &emoji_name).to_base64();
						let emoji = guild_id.create_emoji(&ctx.http, &emoji_name, &image).await.expect("Could not create emoji on server.");
						let emoji_id = emoji.id;
						msg.channel_id.say(&ctx.http, format!("<:{emoji_name}:{emoji_id}>")).await.expect("Could not send emoji message.");
						msg.channel_id.say(&ctx.http, format!(":{emoji_name}:")).await.expect("Could not send emoji shortcode.");
						return;
					} else {
						let target = format!("{GSTATIC_PREFIX}{revision}/{last}/{last}_{first}.png");
						let response = reqwest::get(target.clone()).await.expect("Could not fetch file.");
						let status = response.status();
						if status.is_success() {
							let content = response.bytes().await.expect("Could not get bytes from response.");
							let emoji_name = format!("{first}_{last}").replace("-", "_");
							let image = CreateAttachment::bytes(content, &emoji_name).to_base64();
							let emoji = guild_id.create_emoji(&ctx.http, &emoji_name, &image).await.expect("Could not create emoji on server.");
							let emoji_id = emoji.id;
							msg.channel_id.say(&ctx.http, format!("<:{emoji_name}:{emoji_id}>")).await.expect("Could not send emoji message.");
							msg.channel_id.say(&ctx.http, format!(":{emoji_name}:")).await.expect("Could not send emoji shortcode.");
							return;
						}
					}
				}
			}
		}
	}
}

#[tokio::main]
async fn main() {
	let token = fs::read_to_string("token").expect("Expected a plaintext Discord API token at `./token`");
	let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_EMOJIS_AND_STICKERS;
	let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client.");
	if let Err(error) = client.start().await {
		eprintln!("Client error: {error:?}");
	}
}