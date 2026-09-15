# Development

Cargo workspace with three members:

```
blog/       # Dioxus site (SSG via dioxus-fullstack) — the live site
content/    # site-content crate: shared markdown parsing + models
site-egui/  # eframe/egui experiment — the future site
```

### Serving the Dioxus blog

Requires the dioxus-cli (uncomment it in devenv.nix or `cargo install dioxus-cli`):

```bash
cd blog && dx serve
```

### Serving the egui experiment

```bash
cd site-egui && trunk serve
```

Trunk builds for `wasm32-unknown-unknown` and serves `index.html` with the wasm bound to `#site_canvas`.

### Tailwind (blog only)

```bash
cd blog
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

