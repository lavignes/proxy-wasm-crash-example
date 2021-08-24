FROM envoyproxy/envoy-debug:v1.19.0

ARG NAME

COPY envoy.yaml /envoy.yaml
COPY proxy_wasm_crash_example.wasm /

CMD envoy --log-level debug -c /envoy.yaml

