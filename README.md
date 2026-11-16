# kowkodivka.github.io

My personal website, built with [Rust](https://rust-lang.org/) and [Dioxus](https://dioxuslabs.com/).

## Prerequisites

- [Rust](https://rustup.rs/)
- The WebAssembly target:

  ```sh
  rustup target add wasm32-unknown-unknown
  ```

- The Dioxus CLI:

  ```sh
  cargo binstall dioxus-cli
  ```

## Developing

Start the dev server with hot reloading:

```sh
dx serve --platform web
```

The site will be available at <http://localhost:8080>. Changes to `rsx!` markup and assets are hot-reloaded; changes to Rust logic trigger a fast rebuild.

## Building

Build an optimized production bundle:

```sh
dx bundle --platform web --release
```

The static output is placed in `target/dx/kowkodivka/release/web/public/`
and can be served by any static file host. To preview it locally:

```sh
python -m http.server -d target/dx/kowkodivka/release/web/public 8000
```

### Deployment

The site is deployed to GitHub Pages via GitHub Actions on every push to `main`. Since the site uses client-side routing, `index.html` is copied to `404.html` so that direct links to subpages work.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE.md](LICENSE.md) file for details.
