use std::fs;
// use std::io;

use serenity::all::CreateAttachment;
use unicode_segmentation::UnicodeSegmentation;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

const API_PREFIX: &str = "https://emojik.vercel.app/s/";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content == "!ping" {
			if let Err(error) = msg.channel_id.say(&ctx.http, "pong!").await {
				eprintln!("Error sending message: {error:?}");
			}
		} else {
			let shortcodes = msg
				.content
				.replace(" ", "")
				.graphemes(true)
				.map(|g| {
					emojis::get(g).and_then(|e| e.shortcode())
				})
				.collect::<Vec<Option<&str>>>();

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

			let guild_id = msg.guild_id.unwrap();
			if let [first, last] = &code_points[..] {
				let mut emojis = guild_id.emojis(&ctx.http).await.expect("Could not fetch emojis.");
				if emojis.len() == 50 {
					emojis.sort_by(|a, b| {
						a.id.cmp(&b.id)
					});
					let emoji = emojis.first().unwrap();
					guild_id.delete_emoji(&ctx.http, emoji.id).await.expect("Could not delete emoji.");
					emojis = guild_id.emojis(&ctx.http).await.expect("Could not fetch emojis.");
				}
				for emoji in emojis {
					let emoji_name = if let [Some(short_first), Some(short_last)] = &shortcodes[..] {
						format!("{short_first}_{short_last}")
					} else {
						format!("{first}_{last}").replace("-", "_")
					};
					if emoji.name == emoji_name {
						let emoji_id = emoji.id;
						msg.channel_id.say(&ctx.http, format!("<:{emoji_name}:{emoji_id}>")).await.expect("Could not send emoji message.");
						msg.channel_id.say(&ctx.http, format!(":{emoji_name}:")).await.expect("Could not send emoji shortcode.");
						return;
					} else {
						let emoji_name = if let [Some(short_first), Some(short_last)] = &shortcodes[..] {
							format!("{short_last}_{short_first}")
						} else {
							format!("{last}_{first}").replace("-", "_")
						};
						if emoji.name == emoji_name {
							let emoji_id = emoji.id;
							msg.channel_id.say(&ctx.http, format!("<:{emoji_name}:{emoji_id}>")).await.expect("Could not send emoji message.");
							msg.channel_id.say(&ctx.http, format!(":{emoji_name}:")).await.expect("Could not send emoji shortcode.");
							return;
						}
					}
				}
				let target = format!("{API_PREFIX}/{first}_{last}");
				let response = reqwest::get(target.clone()).await.expect("Could not fetch png.");
				let status = response.status();
				if status.is_success() {
					let content = response.bytes().await.expect("Could not get bytes from response.");
					let emoji_name = if let [Some(short_first), Some(short_last)] = &shortcodes[..] {
						format!("{short_first}_{short_last}")
					} else {
						format!("{first}_{last}").replace("-", "_")
					};
					let image = CreateAttachment::bytes(content, &emoji_name).to_base64();
					let emoji = guild_id.create_emoji(&ctx.http, &emoji_name, &image).await.expect("Could not create emoji on server.");
					let emoji_id = emoji.id;
					msg.channel_id.say(&ctx.http, format!("<:{emoji_name}:{emoji_id}>")).await.expect("Could not send emoji message.");
					msg.channel_id.say(&ctx.http, format!(":{emoji_name}:")).await.expect("Could not send emoji shortcode.");
					return;
				} else {
					let target = format!("{API_PREFIX}/{last}_{first}");
					let response = reqwest::get(target.clone()).await.expect("Could not fetch png.");
					let status = response.status();
					if status.is_success() {
						let content = response.bytes().await.expect("Could not get bytes from response.");
						let emoji_name = if let [Some(short_first), Some(short_last)] = &shortcodes[..] {
							format!("{short_last}_{short_first}")
						} else {
							format!("{last}_{first}").replace("-", "_")
						};
						let image = CreateAttachment::bytes(content, &emoji_name).to_base64();
						let emoji = guild_id.create_emoji(&ctx.http, &emoji_name, &image).await.expect("Could not create emoji on server.");
						let emoji_id = emoji.id;
						msg.channel_id.say(&ctx.http, format!("<:{emoji_name}:{emoji_id}>")).await.expect("Could not send emoji message.");
						msg.channel_id.say(&ctx.http, format!(":{emoji_name}:")).await.expect("Could not send emoji shortcode.");
						return;
					}
				}
				msg.channel_id.say(&ctx.http, format!("No combination")).await.expect("Could not send unsuccessful message.");
			} else if msg.attachments.len() > 0 {
				for attachment in msg.attachments {
					let target = attachment.url;
					let response = reqwest::get(target.clone()).await.expect("Could not fetch attachment.");
					let status = response.status();
					if status.is_success() {
						let content = response.bytes().await.expect("Could not get bytes from response.");
						let file_name = std::path::Path::new(&attachment.filename);
						let emoji_name = file_name.file_stem().unwrap().to_string_lossy().to_ascii_lowercase().replace(|c| !char::is_alphanumeric(c), "_");
						let image = CreateAttachment::bytes(content, &emoji_name).to_base64();
						guild_id.create_emoji(&ctx.http, &emoji_name, &image).await.expect("Could not create emoji on server.");
						msg.channel_id.say(&ctx.http, format!(":{emoji_name}:")).await.expect("Could not send emoji shortcode.");
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