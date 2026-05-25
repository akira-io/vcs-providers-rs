# Telemetry

Telemetry wraps transport execution without knowing concrete HTTP clients or providers.

```rust
let recorder = telemetry().recorder();
let transport = telemetry()
    .transport(transport)
    .sink(recorder.clone())
    .build();
```

## Events

Core telemetry currently emits:

- Request started
- Request finished

Request events include method and URL. Response events include status code and duration in milliseconds.

## Sink Contract

`TelemetrySink` receives `TelemetryEvent` values. Applications can bridge this into tracing, OpenTelemetry, metrics collectors, logs, or tests.

Telemetry types stay provider-neutral and do not expose transport implementation details.
