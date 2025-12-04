
local M = {}

function M.test()
    print("running test.test")
    info("called test.test")
end

function M.start()
    install_module("/data/adb/modules/test/module.zip")
end

function M.set()
    setConfig("wode","adwfihiowafhoiwhqfoi")
end
function M.get()
	local dd=getConfig("wode")
	info(dd)
	print(dd)
end
function M.lsp()
	modules.zygisk_lsposed.test()-- exaplme to call zygisk_lspoed function
end
function M.extend()-- extend to other module to call 
    local hello = require("hello")
    hello.say_hello()  --call rust function
end

return M