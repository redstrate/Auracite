# Auracite

Export your FFXIV character in portable, generic formats.

In the eventual future, the official servers will shut down and the Lodestone will disappear. This tool exports your
character's data in machine-readable JSON (so it can be imported by something else) and a Lodestone-like HTML page which
you can display in your browser.

## Usage

This tool makes several HTTP requests to the Lodestone, but they currently are only a few. The tool does not contact any
3rd-party external services.

### Web

Auracite can run inside your web browser, accessed at [auracite.xiv.zone](https://auracite.xiv.zone/). It has the same features as the regular version.

### Desktop

A desktop version is available, just run `cargo run`. There is currently no binary distribution available.

### CLI

Provide a character name via `--name`:

```shell
auracite --name "John Doe" 
```

To enable support for the Dalamud plugin, add `--dalamud`.

## Building

### Desktop

To run the desktop client, simply run `cargo run`. You must have Qt6 development packages installed beforehand.

### Flatpak

To build the Flatpak, use `flatpak-builder` or the helper script `scripts/build-flatpak.sh`. An `auracite.flatpak` file will be generated.

### Web

To build the Web version, use `wasm-pack` or the helper script `scripts/build-web.sh`. A folder called `pkg/` will be generated, and the HTML files live in `web/`.

### Dalamud Mode

Auracite can only collect so much data from the Lodestone, some data can only be collected when logged in. To do this,
we provide a Dalamud plugin to run alongside the tool. The plugin is currently available
[in my personal Dalamud repository](https://github.com/redstrate/DalamudPlugins). The plugin can be
safely removed if you're done using Auracite.

## Supported Data

| Data                      | Supported | Notes                                                                             |
|---------------------------|-----------|-----------------------------------------------------------------------------------|
| Name                      | ✅         |                                                                                   |
| World/Data Center         | ✅         |                                                                                   |
| Race/Subrace/Gender       | ✅         |                                                                                   |
| City-state                | ✅         |                                                                                   |
| Nameday                   | ✅         |                                                                                   |
| Guardian                  | ✅         |                                                                                   |
| Portrait/Full-body Images | ✅         | These are the images displayed on the Lodestone.                                  |
| Playtime                  | ✅         | Requires the Dalamud plugin.                                                      |
| Currencies                | ⭕️        | Only gil is supported, and requires the Dalamud plugin.                           |
| Appearance Data           | ✅         | Requires the Dalamud plugin.                                                      |
| Adventurer Plate          | ✅         | Requires the Dalamud plugin.                                                      |
| Misc. state               | ⭕️        | Mentor and novice status, also player commendations. Requires the Dalamud plugin. |

Currently, more types of data is planned to be supported in the future.

## License

This project is licensed under the [GNU Affero General Public License 3](LICENSE).
