# Work in Progress

This is as much a learning exercise for me as it is a tool that I am going to use

## To test:

```sh
printf "Content-Length: 85\r\n\r\n{\"jsonrpc\": \"2.0\", \"method\": \"initialize\", \"id\": 1, \"params\": {\"capabilities\": {}}}" | cargo run
```

## Intro

A minimal example LSP server that can only respond to the `gotoDefinition` request. To use
this example, execute it and then send an `initialize` request.

```
Content-Length: 85

{"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {"capabilities": {}}}
```

This will respond with a server response. Then send it a `initialized` notification which will
have no response.

```
Content-Length: 59

{"jsonrpc": "2.0", "method": "initialized", "params": {}}
```

Once these two are sent, then we enter the main loop of the server. The only request this
example can handle is `gotoDefinition`:

```
Content-Length: 159

{"jsonrpc": "2.0", "method": "textDocument/definition", "id": 2, "params": {"textDocument": {"uri": "file://temp"}, "position": {"line": 1, "character": 1}}}
```

To finish up without errors, send a shutdown request:

```
Content-Length: 67

{"jsonrpc": "2.0", "method": "shutdown", "id": 3, "params": null}
```

The server will exit the main loop and finally we send a `shutdown` notification to stop
the server.

```
Content-Length: 54

{"jsonrpc": "2.0", "method": "exit", "params": null}
```
