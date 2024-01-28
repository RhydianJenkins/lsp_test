# LSP Server in Rust

A minimal LSP server listening for codeAction events over tcp.

## Use (Server)

To listen for tcp requests, start the server by running the following:

```sh
# build
cargo build --release

# run
./target/release/lsp_test
```

## Use (nc)

With the server running, you can use `nc` to hit the server over TCP:

```sh
echo '{"jsonrpc": "2.0", "method": "textDocument/codeAction", "params": {}, "id": 1}' | nc 127.0.0.1 8080
```

## Use (Neovim)

An example neovim lua consumption:

```lua
-- init.lua
local lspconfig = require("lspconfig")
local configs = require("lspconfig.configs")

if not configs.lsp_test then
    configs.lsp_test = {
        default_config = {
            cmd = { "lsp_test", "--stdio" },
            cmd_cwd = "/path/to/lsp_test/target/release",
            filetypes = { "lua" },
        },
    }
end

lspconfig.lsp_test.setup({})
```
