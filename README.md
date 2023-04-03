# tonic-based-grpc-payment

this is a very simple grpc-payments

inspired by: https://github.com/letsgetrusty/grpc_example

run server:
```sh
cargo run --bin payments-server
```

run client:
```sh
cargo run --bin payments-client
```
and then input the corresponding information as prompted

This will output:
```js
Response=Response { metadata: MetadataMap { headers: 
{"content-type": "application/grpc", 
"date": "XXX", 
"grpc-status": "0"} },
message: BtcPaymentResponse { 
    successful: true, 
    message: "Send XXX BTC to XXX." 
}, extensions: Extensions }
```

and output for server terminal:
```js
We have request: Request { metadata: MetadataMap { headers: 
{"te": "trailers",
"content-type": "application/grpc", 
"user-agent": 
"tonic/0.7.2"} }, 
message: BtcPaymentRequest { 
    from_addr: "XXX", 
    to_addr: "XXX", 
    amount: XXX 
}, extensions: Extensions }
```

XXX is based on yourself
