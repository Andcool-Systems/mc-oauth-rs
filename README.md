# Minecraft OAuth Provider

## 🚀 Description
MC-OAuth-rs is an all-in-one solution that allows users to log in to a website using their Minecraft account without entering their username or password.  

It’s secure, simple, and user-friendly, enabling logins only for users with a licensed Minecraft account.  
The service supports Minecraft versions above 1.8.

## 💡 How it works?
Our service provides an authentication system that requires no complex actions from users or developers.  
All you need to do is join a Minecraft server and get a 6-digit code, then enter it on the website that uses this service.

### 🔑 Features
- 📋 Get username, UUID, and skin through REST API
- 🛡️ Zero Trust (it is impossible to fake Minecraft account data)
- ⚡ Easy to use and implement
- 🔒 The server only processes the authentication process and nothing more

## 💻 For Users
Websites that use this service will ask you for a 6-digit code, which you can get by joining the Minecraft server `auth.mc-oauth.andcool.ru` (or others), then entering it on the website.  
**The website developer will get access to your username, UUID, and skin.**

## 🛠️ For Developers
To integrate this service, add a form on your website for entering the 6-digit code. After the user enters the code, make a request to the API endpoint described below.

### 📡 API Endpoint
```
GET /code/<6-digit code>
```

### Example of a successful response from the server:
```json
{
    "nickname": "AndcoolSystems",
    "UUID": "1420c63cb1114453993fb3479ba1d4c6",
    "properties": [
        {
            "name": "textures",
            "signature": "<base64 string; signed data using Yggdrasil's private key>",
            "value": "<base64 string>"
        }
    ]
}
```
*You can read more about the data inside `properties` [here](https://minecraft.wiki/w/Mojang_API#Query_player's_skin_and_cape).*

> [!NOTE]
> The code is valid for 5 minutes after issuance and can only be used once.

## 🛠️ Local Deployment
To run this service locally, first install [Rust](https://www.rust-lang.org/tools/install).  

Then run these commands:
```shell
git clone https://github.com/Andcool-Systems/mc-oauth-rs.git
cd mc-oauth-rs

cargo build --release
```

After the project is fully built, the binary file for your OS will be available at `./target/release/mc-oauth.*`.

### 📋 Configuration
For the server to work, create a `config.toml` file in the same directory as the executable with the following contents:

```toml
[global]
# Server logging level. Possible values:
# debug, info, warn, error, trace
logging_level = "info"

[api]
# API address
addr = "0.0.0.0"

# API port
port = 8008

# Life time of assigned code
code_life_time = 300

# Length of generated code
code_length = 6

[server]
# Minecraft server address
addr = "0.0.0.0"

# Minecraft server port
port = 25565

# Server connection timeout
# Sets the maximum time a client can stay connected to the server. Used to prevent idle or junk connections.
timeout = 10

# [Optional] This parameter defines the server host to which the clients connect.
# This is necessary to prevent proxy bypass or spoofing.
server_ip = "localhost"

# Enable partial support of 1.19-1.19.2 clients
# https://github.com/Andcool-Systems/mc-oauth-rs/issues/1
support_1_19 = false

[server.config]
# Minecraft server name (used for internal auth; can be empty)
server_name = "mc-oauth-rs"

# Protocol version (`0` for auto)
# If set to 0, the protocol version that the client uses will be applied.
protocol = 0

# Minecraft version string (In fact, it's just a stub)
version = "1.21"

# Session Auth URL  
# `{{NAME}}` in string will be replaced by the client nickname  
# `{{HASH}}` will be replaced by the generated client hash
auth_url = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={{NAME}}&serverId={{HASH}}"

[server.status]
# Server description (you can use MOTD)
description = "§6mc-oauth.andcool.ru"

# Max players count, displayed in server list
players_max = 0

# Online players count, displayed in server list
players_online = 0

# Path to the server icon (can be empty)
# Should be in .png format and 64x64 pixels in size
icon_path = "server_icon.png"

[messages]
# Message for successful auth  
# `{{NAME}}` will be replaced by the client nickname  
# `{{UUID}}` will be replaced by the client UUID  
# `{{CODE}}` will be replaced by the generated code
success = "Hello, §6{{NAME}}§r! Your code is: §a{{CODE}}"

# Message for Mojang API error
bad_session = "§cFailed to login: Invalid session (Try restarting your game and the launcher)"

# Message for bad server address (`server_ip` setting)
using_proxy = "§cYou are using a proxy!"

# Message for internal server error
internal_error = "§cSorry, internal server error occurred"

# Message for unsupported client version
unsupported_client_version = "§cYou are using unsupported version of client!"
```


### 🚀 Running
After configuring, run the compiled binary file through the console.

## Known issues
- For Minecraft 1.19.* clients, encryption does not work correctly

---
**Created by AndcoolSystems with ❤, 3 February, 2025**
