# Eagle

🦅 A game engine for academic large-scale experiments.

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg?style=flat)
[![GitHub Repo stars](https://img.shields.io/github/stars/oxpt/eagle?style=social&color=yellow)](https://github.com/oxpt/eagle)

⚠️ **Currently under development. Not for use**

## Concepts

- Easy to implement, programme, composite, and host games.
- Consistent and replayable game play.
- Reliable based on the type-centric design.
- Scalable to large number of players.
- High throughput and low latency.
- Secure by default.
- Platform agnostic.

## Development

```sh
tools/wasm-pack.sh build
tools/wrangler.sh dev --var "ALLOWED_ORIGINS:*"
tools/next.sh dev
```

## License

Dual-licensed under:

- [MIT license](https://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](https://opensource.org/licenses/Apache-2.0).

(You can pick either of the two, at your option.)
