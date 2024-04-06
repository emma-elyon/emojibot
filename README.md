# Emma's Emoji

Allows combining emoji using Gboard combinators by uploading the resulting
sticker as an emoji to a discord server.

## To do

* Prune oldest half of server's emojis when limit is hit
* Check for duplicates through discord API before reqwesting gstatic
* Edit original sender's message to also include the new emoji and shortcode
  instead of responding with an entire bot message
* Globally store successful revisions for each combinations to avoid hammering
  the gstatic server 27×2 times (worst case) before returning
* Allow converting arbitrary pictures to emoji
* Better emoji names

## Emoji shortcode

https://api.github.com/emojis

## gstatic revisions

https://git.emoji.supply/kitchen/emojidata.js
