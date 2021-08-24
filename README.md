Build wasm:

```
cargo build --target wasm32-unknown-unknown
```

Run envoy listening on http://localhost:8080:
```
./run-envoy.sh
```

Curl envoy:
```
curl -H 'Host: www.example.com' http://localhost:8080
```
