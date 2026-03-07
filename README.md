# Auracite

Export your FFXIV character in portable, generic formats. This includes
data in machine-readable JSON (and can be imported by other programs
like [Kawari](https://github.com/redstrate/Kawari)) and a Lodestone-esque
HTML page which you can display in your browser.

## Usage

Auracite runs inside your web browser, and can be accessed at [auracite.xiv.zone](https://auracite.xiv.zone/).

## Building

Use `wasm-pack` or the helper script `scripts/build-web.sh`. A folder called `pkg/` will be generated, and the HTML files live in `web/`.

### Dalamud Mode

Auracite can only collect so much data from the Lodestone, some data can only be collected when logged in. To do this,
we provide a Dalamud plugin to run alongside the tool. The plugin is currently available
[in my personal Dalamud repository](https://github.com/redstrate/DalamudPlugins). The plugin can be
safely removed if you're done using Auracite.

## License

This project is licensed under the [GNU Affero General Public License 3](LICENSE).
