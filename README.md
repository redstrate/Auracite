# Auracite

Export your FFXIV character in portable, generic formats. This includes
data in machine-readable JSON (and can be imported by other programs
like [Kawari](https://github.com/redstrate/Kawari)) and a Lodestone-esque
HTML page which you can display in your browser.

## Usage

### Web

Auracite can run inside your web browser, accessed at [auracite.xiv.zone](https://auracite.xiv.zone/). It works exactly the same as the regular version.

### Desktop

A desktop version is available, just run `cargo run`. There is currently no binary distribution available.

### CLI

Provide a character name via `--name`:

```shell
auracite --name "John Doe" 
```

Or provide a Lodestone id:

```shell
auracite --id 9001
```

To enable support for the Dalamud plugin, add `--dalamud`.

## Building

### Desktop

To run the desktop client, simply run `cargo run`. You need the Qt6 development packages installed beforehand.

### Flatpak

To build the Flatpak, use `flatpak-builder` or the helper script `scripts/build-flatpak.sh`. An `auracite.flatpak` file will be generated.

### Web

To build the Web version, use `wasm-pack` or the helper script `scripts/build-web.sh`. A folder called `pkg/` will be generated, and the HTML files live in `web/`.

### Dalamud Mode

Auracite can only collect so much data from the Lodestone, some data can only be collected when logged in. To do this,
we provide a Dalamud plugin to run alongside the tool. The plugin is currently available
[in my personal Dalamud repository](https://github.com/redstrate/DalamudPlugins). The plugin can be
safely removed if you're done using Auracite.

## License

This project is licensed under the [GNU Affero General Public License 3](LICENSE).
