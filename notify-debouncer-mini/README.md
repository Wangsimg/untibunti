# Notify debouncer

[![» Docs](https://flat.badgen.net/badge/api/docs.rs/df3600)][docs]

Tiny debouncer for [notify]. Filters incoming events and emits only one event per timeframe per file.

## Features

- `crossbeam` enabled by default, for crossbeam channel support.  
This may create problems used in tokio environments. See [#