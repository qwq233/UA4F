# UA4F

[中文](./README_CN.md)

Another User Agent faker, allowing users to bypass multi device detection for Campus Network via socks5 proxy.

Inspired by [UA3f](https://github.com/SunBK201/UA3F)

# Features

- Better performence ~~(i guess)~~
- Fully written in ***rust***
- Easy to migrate from ua3f
- Compatible with clash

# Principle

It will first grab the first few bytes of the first packet to check if it is http traffic. If so, it will continue to grab a total of about 4k of traffic and modify the User Agent in it.

# Install

## Using prebuilt package

You can find most common pre-built packages in [Release](https://github.com/qwq233/UA4F/releases).

Download and install it. Then you are ready to use it.

## Build Manually

1. Make sure you have installed latest **nightly** version of rust toolchain and the target corresponding to the target platform. (E.G: `x86_64-unknown-linux-musl` for x86_64 OpenWrt platform or `x86_64-unknown-linux-gnu` for x86_64 GNU/Linux)

2. Clone this project

3. Build with `cargo`

4. You are ready to use. The build result is usually located in `target/TARGET_ARCH/release/ua4f`

#### Example Command

```shell
# Assuming you are using rustup to manage rust toolchain
rustup default nightly
rustup add x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-musl
```

## Build with OpenWrt

1. Make sure you have installed latest **nightly** version of rust toolchain and the target corresponding to the target platform and use musl as the C library. (E.G: `x86_64-unknown-linux-musl` for x86_64 platform)

2. Clone this project into the `package/ua4f`

3. Enable building luci-compat and ua4f package

4. You are ready to build the image or build the package separately.

#### Example Command

```shell
# Assuming you are using rustup to manage rust toolchain
rustup default nightly
rustup target add x86_64-unknown-linux-musl

echo "CONFIG_PACKAGE_ua4f=y" >> .config
make defconfig

# Build this package only
make package/ua4f/{clean,compile} V=s -j$(nproc)
# or include in the image
make -j$(nproc)
```

# Configure

## By Luci

It can easily configured via luci. The configuration page is located at `Service -> UA4F`

## By command

Assuming you are using OpenWrt. You can run these command to configure and start it as background service.

```shell
# Custom UA
uci set ua4f.main.ua="Mozilla/5.0 (Window NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/555.66"
# Listen port
uci set ua4f.main.port="1080"
# Bind address
uci set ua4f.main.bind="127.0.0.1"
# Log level
uci set ua4f.main.log_level="info"

# Apply
uci commit ua3f

# Start service
service ua4f enable
service ua4f start
```

## By systemd

For regular distributions (Like ***Arch Linux***), you will most likely use systemd to manage background services.

In this situation, you need to write the systemd service file. But don't worry, I have provided an example file and just modify it according to your needs.

1. Create file in `/etc/systemd/system/multi-user.target.wants/ua4f.service`

2. Copy these lines below and modify it according your needs.

```ini
[Unit]
Description=Another User Agent faker, allowing users to bypass multi device detection for Campus Network via socks5 proxy.
Documentation=https://github.com/qwq233/ua4f/
After=network.target

[Install]
WantedBy=multi-user.target

[Service]
# Configure part
Environment=bind_address=127.0.0.1
Environment=port=1080
Environment=custom_ua=Mozilla/5.0 (Window NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/555.66
Environment=log_level=info

# Prevent writes to /usr, /boot, and /etc
User=nobody
ProtectSystem=full
# Prevent accessing /home, /root and /run/user
ProtectHome=true

# Allow modify log file
ReadWritePaths=-/var/log/

Type=simple

ExecStart=/path/to/ua4f -b "$bind_address" -l "$log_level" -p "$port" -f "$custom_ua"
KillSignal=SIGTERM
Restart=on-abort
RestartSec=5s

```

3. Enable and start the service

#### Example Command

```shell
# Download the unit file (same content as above)
curl https://github.com/qwq233/UA4F/raw/refs/heads/master/misc/ua4f.service -0 -o /etc/systemd/system/multi-user.target.wants/ua4f.service

# Edit the file
vim /etc/systemd/system/multi-user.target.wants/ua4f.service

# Enable and start the service
systemctl enable ua4f.service
systemctl start ua4f.service
```

# License

**AGPL-3.0-or-later**

```
UA4F - Another User Agent faker
Copyright (C) 2024  James Clef <qwq233@qwq2333.top>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

# Credit

Some luci code from [UA3F](https://github.com/SunBK201/UA3F)

License: `GPL-3.0-only`
```
UA3f
Copyright (C) 2024  SunBK201

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
