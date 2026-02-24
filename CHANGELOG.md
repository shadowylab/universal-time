# Changelog

<!-- All notable changes to this project will be documented in this file. -->

<!-- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), -->
<!-- and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). -->

<!-- Template

## Unreleased

### Breaking changes

### Changed

### Added

### Fixed

### Removed

### Deprecated

-->

## Unreleased

### Breaking changes

- Remove `set_global_time_context` function (https://github.com/shadowylab/universal-time/pull/1)
- Link errors if the time provider is missing (https://github.com/shadowylab/universal-time/pull/1)
- Require `define_time_provider!` macro on `no_std`/`WASM-unknown` (https://github.com/shadowylab/universal-time/pull/1)

## v0.1.0 - 2026/02/23

First release.
