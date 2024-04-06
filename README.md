# Simple WoL bot, built on top of teloxide

# Installation
1. Edit systemd service to ```/etc/systemd/system/wol.service```.
2. Create config in workdir.
3. Enable and start it.

# Example ```config.json```
```json
{
  "bot_token": "telegram bot token here",
  "ip": "192.168.0.X",
  "mac": "A1:B2:C3:D4:E5:F6",
  "admin_id": 123456789
}
```

You can get bot token from https://t.me/BotFather.

User id from https://t.me/userinfobot.

Get your ip and mac address from pc, that you want to turn on.

# Example systemd service

```unit file (systemd)
[Unit]
Description=Run the Wake-On-Lan (WOL) bot
After=network.target

[Service]
Type=simple
ExecStart=PATH_TO_WTP_BIN
WorkingDirectory=PATH_TO_WORKDIR
Restart=always
RestartSec=5
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=wol

[Install]
WantedBy=multi-user.target
```