# LSP Server in Rust

A minimal LSP server listening for codeAction events over tcp.

## Use (Server)

To listen for tcp requests, start the server by running the following:

```sh
cargo run -- --stdio
```

## Use (Client)

With the server running, you can use `nc` to hit the server over TCP:

```sh
echo '{"jsonrpc": "2.0", "method": "textDocument/codeAction", "params": {}, "id": 1}' | nc 127.0.0.1 8080
```

An example lua consumption

```lua
-- init.lua
local function start_language_server()
    local client_id = vim.lsp.start_client({
        cmd = { "cargo", "run", "--", "--stdio" },
        cmd_cwd = "/path/to/lsp_test",
    })

    if not client_id then
        print("Failed to start language server")
        return
    end

    vim.lsp.buf_attach_client(0, client_id)
end

vim.api.nvim_create_autocmd("FileType", {
    pattern = "lua",
    callback = function()
        start_language_server()
    end,
})
```
