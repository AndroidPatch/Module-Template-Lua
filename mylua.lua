
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
    local hello = require("libhello")
	print(hello.say_hello("world"))--call rust function or C function
    --hello.say_hello()  
end
function M.version() --module version(recommended addition)
return 1
end

function M.post_data_fs()
    info("pre-mount called")
end

function M.post_mount()
    info("post-mount called")
end
function M.service()
    info("service called")
end

return M