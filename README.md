# QoE CLI Runner

A command-line tool for running and analyzing adaptive bitrate (ABR) streaming simulations using a Rust backend.

This CLI interfaces with the core simulation logic from [`qoe_core`](https://github.com/yourname/qoe-core) and outputs performance scores for different ABR strategies.

---

## Features

- Run simulations for:
  - Fixed Bitrate
  - Buffer-Based
  - Throughput-Based
- Export results as CSV and JSON
- CLI flags, aliases, and positional arguments
- Customizable throughput window
- Clean modular design and tested core

---

## Usage

### Run all strategies:

```bash
cargo run --release
```

### Run a single strategy:

```bash
cargo run --release -- fixed
cargo run --release -- buffer
cargo run --release -- throughput -w 4
```

### Export results:

```bash
cargo run --release -- all --json scores.json -o scores.csv
```

---

## CLI Options

| Flag        | Description                                      |
|-------------|--------------------------------------------------|
| `<ABR>`     | Positional argument: `fixed`, `buffer`, `throughput`, or `all` |
| `-o, --output` | Output CSV file (default: `scores.csv`)        |
| `-w, --window` | Throughput window size (only for throughput ABR) |
| `--json`    | Optional path to export JSON file                |

---

## Example Output

```
Running QoE batch tests...

ABR: Fixed           | Score: 52.3 | Bitrate: 1100 kbps | Stalls: 2
ABR: BufferBased     | Score: 63.4 | Bitrate: 1400 kbps | Stalls: 1
ABR: ThroughputBased | Score: 78.9 | Bitrate: 1800 kbps | Stalls: 0

✅ Results written to scores.csv
📝 JSON exported to scores.json
```

---

## Testing

Unit tests included:

```bash
cargo test
```

Covers:
- All ABR strategies
- Configuration builder
- Result generation

---

## Project Structure

```
src/
├── main.rs       # CLI entrypoint
├── types.rs      # CLI args, ScoreRecord, enums
├── config.rs     # Builds SimulationConfigs from args
├── runner.rs     # Runs simulations and returns results
└── io.rs         # Handles CSV and JSON export
```

---

## Credits

- Core simulation engine by [`qoe-core`](https://github.com/mrjaywilson/qoe-core)
- CLI design & polish by [@mrjaywilson](https://github.com/mrjaywilson)

---

## License

MIT
