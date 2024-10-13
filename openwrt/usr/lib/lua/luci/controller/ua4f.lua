module("luci.controller.ua4f", package.seeall)

function index()
	entry({ "admin", "services", "ua4f" }, cbi("ua4f"), "UA4F", 1)
end