# Simple WoL bot, built on top of teloxide

# Installation without docker
1. Edit systemd service to ```/etc/systemd/system/wol.service```.
2. Create config in ```config/config.json```.
3. Enable and start it.

# Installation with docker
1. Create a directory and a ```compose.yml``` file in it.
2. Edit config in path ```config/config.json```.
3. Start it with ```docker compose up -d```.

# Example ```config.json```
```json
{
  "bot_token": "telegram bot token here",
  "ip": "192.168.0.X",
  "mac": "A1:B2:C3:D4:E5:F6",
  "admin_id": 123456789
}
```

# Example ```compose.yml```
```yaml
services:
  bot:
    image: ghcr.io/linuxfight/wol_bot:master
    volumes:
      - ./config:/app/config
    restart: unless-stopped
    network_mode: host
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