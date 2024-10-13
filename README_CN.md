# UA4F

另一个User-Agent伪装工具，允许用户通过socks5代理绕过校园网络的多设备检测。

受 [UA3f](https://github.com/SunBK201/UA3F) 启发

# 特点

- 更好的性能__*大概*__
- 完全使用 ***rust*** 编写
- 易于从ua3f迁移
- 兼容clash

# 原理

它将首先抓取第一个数据包的前几个字节，以检查是否为http流量。如果是，它将继续抓取大约4k的流量并修改其中的用户代理。

# 安装

## 使用预构建包

你可以在 [Release](https://github.com/qwq233/UA4F/releases) 中找到大多数常见的预构建包。

下载并安装它。然后你就可以使用它了。

## 手动构建

1. 确保你已安装了最新的 **nightly** 版本的rust工具链和与目标平台对应的目标。（例如：`x86_64-unknown-linux-musl` 适用于x86_64 OpenWrt平台，或 `x86_64-unknown-linux-gnu` 适用于x86_64 GNU/Linux）

2. 克隆此项目

3. 使用 `cargo` 构建

4. 你已准备好使用。构建结果通常位于 `target/TARGET_ARCH/release/ua4f`

#### 示例命令

```shell
# 假设你使用rustup管理rust工具链
rustup default nightly
rustup add x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-musl
```

## 与OpenWrt构建

1. 确保你已安装了最新的 **nightly** 版本的rust工具链和与目标平台对应的目标，并使用musl作为C库。（例如：`x86_64-unknown-linux-musl` 适用于x86_64平台）

2. 将此项目克隆到 `package/ua4f`

3. 启用构建 luci-compat 和 ua4f 包

4. 你已准备好构建镜像或单独构建包。

#### 示例命令

```shell
# 假设你使用rustup管理rust工具链
rustup default nightly
rustup target add x86_64-unknown-linux-musl

echo "CONFIG_PACKAGE_ua4f=y" >> .config
make defconfig

# 仅构建此包
make package/ua4f/{clean,compile} V=s -j$(nproc)
# 或包含在镜像中
make -j$(nproc)
```

# 配置

## 通过Luci

可以通过Luci轻松配置。配置页面位于 `Service -> UA4F`

## 通过命令

假设你使用OpenWrt。你可以运行以下命令来配置并将其作为后台服务启动。

```shell
# 自定义UA
uci set ua4f.main.ua="Mozilla/5.0 (Window NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/555.66"
# 监听端口
uci set ua4f.main.port="1080"
# 绑定地址
uci set ua4f.main.bind="127.0.0.1"
# 日志级别
uci set ua4f.main.log_level="info"

# 应用
uci commit ua3f

# 启动服务
service ua4f enable
service ua4f start
```

## 通过 Systemd

对于常规发行版（如 ***Arch Linux***），你很可能会使用systemd来管理后台服务。

在这种情况下，你需要编写systemd服务文件。但不用担心，我已提供了一个示例文件，只需根据你的需求进行修改。

1. 在 `/etc/systemd/system/multi-user.target.wants/ua4f.service` 中创建文件

2. 复制下面的这些行并根据你的需求进行修改。

```
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

3. 启用并启动服务

#### 示例命令

```shell
# 下载Unit文件（内容与上面相同）
curl https://github.com/qwq233/UA4F/raw/refs/heads/master/misc/ua4f.service -0 -o /etc/systemd/system/multi-user.target.wants/ua4f.service

# 编辑文件
vim /etc/systemd/system/multi-user.target.wants/ua4f.service

# 启用并启动服务
systemctl enable ua4f.service
systemctl start ua4f.service
```

# 许可

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

# 鸣谢

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
