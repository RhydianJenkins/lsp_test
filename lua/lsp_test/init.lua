local M = {}

local json = require("lua.lsp_test.json")

local function handle_code_action(request)
    print("lsp_test.handle_code_action", request)

    local response = {
        jsonrpc = "2.0",
        id = 2,
        result = {
            {
                title = "Print 'hello'",
                kind = "quickfix",
                command = {
                    title = "Run Command",
                    command = "runCommand",
                    arguments = { 'vim.print("hello")' },
                },
            },
        },
    }

    return response
end

M.start = function()
    while true do
        local line = io.read()
        if not line or line == "" then
            break
        end

        local request = json.parse(line)
        if not request then
            break
        end

        if request.method == "textDocument/codeAction" then
            local response = handle_code_action(request)
            io.write(json.stringify(response) .. "\n")
            io.flush()
        end
    end
end

return M
