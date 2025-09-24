
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 - 2025-09-23

### Added

- Initial development release of `rs_events`.
- EventEmitter with listener types (`unlimited`, `limited`, `once`).
- Support for tagging & lifetimes.
- Sync and async emission (`tokio`-powered).
- Feature-gated builds (`threaded` vs `no_std/alloc`).
- Robust error handling.
