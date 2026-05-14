# Crucible

A simulated IoT sensor data pipeline built in Rust. Sensors produce readings at configurable intervals and stream them to a central gateway over TCP. The gateway then publishes those readings to consumers via Apache Iggy.

## Architecture

```
[Producer (sensor)] ──TCP──> [Gateway] ──Iggy──> [Consumers]
[Producer (sensor)] ──TCP──> [Gateway] ──Iggy──> [Consumers]
        ...
```

- **Producer** — simulates a sensor of a given type, generates randomized readings at a set frequency, and streams them to the gateway using a length-prefixed binary protocol.
- **Gateway** — TCP server that accepts connections from multiple producers concurrently (one thread per connection). Acts as an [Apache Iggy](https://iggy.rs) producer, forwarding sensor readings into a message stream for downstream consumers.

## Sensor types

| Type          | Output fields                  | Range            |
|---------------|-------------------------------|------------------|
| `temperature` | `id`, `temp`, `unit`          | -10 °C to 42 °C  |
| `humidity`    | `id`, `HUM`                   | 0% to 99.99%     |

## Getting started

```bash
# Start the gateway
cargo run --bin gateway

# Start one or more producers (in separate terminals)
# Usage: producer <sensor_type> <frequency_seconds>
cargo run --bin producer temperature 5
cargo run --bin producer humidity 10
```

The gateway listens on `127.0.0.1:8080`. Each producer connects on startup and sends a reading every `<frequency>` seconds.
