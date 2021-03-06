# sms-telegram-forwarder

Forward your SMS to telegram with `termux-api`.

## Usage

1. Install [Termux](https://f-droid.org/en/packages/com.termux/), [Termux:API](https://f-droid.org/en/packages/com.termux.api), and [Termux:Boot](https://f-droid.org/en/packages/com.termux.boot/).
2. Do `pkg install termux-api` in the Termux App.
3. Download the `sms-telegram-forwarder` binary from the release page and put it in your termux directory `~/.termux/boot/`.
4. Create `~/.termux/boot/forward-sms.sh` with the following content:
```
#!/bin/sh
termux-wake-lock && ./sms-telegram-forwarder --bot-token <your-telegram-bot-token> --chat-id <your-chat-room-id> --interval-seconds 3
```
5. Disable battery optimization for the Termux App.

Then all your new SMS will be forwarded to your telegram chat room.
