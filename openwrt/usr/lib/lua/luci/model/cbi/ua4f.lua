local uci = require("luci.model.uci").cursor()

ua4f = Map("ua4f",
    "UA4F",
    [[
        <a href="https://github.com/qwq233/UA4F" target="_blank">Version: 0.1.0 </a>
        <br>
        Another User Agent faker, allowing users to bypass multi device detection for Campus Network via socks5 proxy.
    ]]
)

enable = ua4f:section(NamedSection, "enabled", "ua4f", "Status")
main = ua4f:section(NamedSection, "main", "ua4f", "Settings")

enable:option(Flag, "enabled", "Enabled")
status = enable:option(DummyValue, "status", "Status")
status.rawhtml = true
status.cfgvalue = function(self, section)
    local pid = luci.sys.exec("pidof ua4f")
    if pid == "" then
        return "<span style='color:red'>" .. "Stopped" .. "</span>"
    else
        return "<span style='color:green'>" .. "Running" .. "</span>"
    end
end

main:tab("general", "General Settings")
main:tab("log", "Log")

port = main:taboption("general", Value, "port", "Port")
port.placeholder = "1080"
bind = main:taboption("general", Value, "bind", "Bind Address")
bind.placeholder = "127.0.0.1"
ua = main:taboption("general", Value, "ua", "User-Agent")
ua.placeholder = "Mozilla/5.0 (Window NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/555.66"
log_level = main:taboption("general", ListValue, "log_level", "Log Level")
log_level:value("debug")
log_level:value("info")
log_level:value("warn")
log_level:value("error")

log = main:taboption("log", TextValue, "")
log.readonly = true
log.cfgvalue = function(self, section)
    return luci.sys.exec("cat /var/log/ua4f.log")
end
log.rows = 30

local apply = luci.http.formvalue("cbi.apply")
if apply then
    io.popen("/etc/init.d/ua4f restart")
end

return ua4f