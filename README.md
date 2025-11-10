## Features & Roadmap
- [x] [Elm's Markers](https://www.esoui.com/downloads/info3395-ElmsMarkers.html) string input
- [x] Zoom/Pan
- [x] Modify marker position
- [x] Change marker icon
- [x] Right click to place new marker
- [x] Sub-zone auto-mapping
- [x] Height-based auto-mapping (Falgravn floors) (partially implemented)
- [x] [Breadcrumbs](https://www.esoui.com/downloads/info3996-Breadcrumbs.html) support (preview only)
- [x] [More (M0R) Markers](https://www.esoui.com/downloads/info4266-MoreMarkers.html) support
- [ ] [Akamatsu format](https://esoui.com/downloads/info3684-Marker.html) support
- [ ] More zones (Dungeons et cetera)?

## Building

Download and install [Rust](https://rustup.rs/)

Install [trunk](https://trunkrs.dev/)
```sh
cargo install --locked trunk
```

Add the WebAssembly target
```sh
rustup target add wasm32-unknown-unknown
```

Build via trunk and serve to localhost
```sh
trunk serve
```