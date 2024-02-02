To test:

```sh
printf "Content-Length: 85\r\n\r\n{\"jsonrpc\": \"2.0\", \"method\": \"initialize\", \"id\": 1, \"params\": {\"capabilities\": {}}}" | cargo run
```
