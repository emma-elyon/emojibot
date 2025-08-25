# Emma's Emoji

Allows combining emoji using Gboard combinators by uploading the resulting
sticker as an emoji to a discord server.

## To do

* Globally store successful revisions for each combinations to avoid hammering
  the gstatic server 27×2 times (worst case) before returning
* Allow entering a custom emoji name after the two emojis to combine
* Allow entering a custom emoji name along with an attached image to convert
* Filter out "not found" emoji from vercel.

## Done

* Prune emoji when hitting 50
* Check for duplicates through discord API before reqwesting gstatic
* Allow converting arbitrary pictures to emoji
* Autogenerate combination shortcode from github shortcode

## Emoji shortcode

https://api.github.com/emojis
