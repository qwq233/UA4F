# UA4F

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

WIP

# Configure

It can easily configured via luci. (WIP)

# License

AGPL-3.0

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

License: GPL-3.0
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
