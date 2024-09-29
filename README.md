# Auracite

Export your FFXIV character in portable, generic formats.

In the eventual future, the official servers will shut down and the Lodestone will disappear. This tool exports your
character's data in machine-readable JSON (so it can be imported by something else) and a Lodestone-like HTML page which
you can display in your browser.

## Usage

Provide a character name via `--name`:

```shell
auracite --name "John Doe" 
```

The tool will create a new folder with the name "John Doe", which will contain the available data. See the table below
for the currently supported data that can be recorded. You may want to check out the `character.html` file that can be
viewed locally in your browser.

This tool makes several HTTP requests to the Lodestone, but they currently are only a few. The tool does not contact any
3rd-party external services.

## Supported Data

| Data                      | Supported | Notes                                            |
|---------------------------|-----------|--------------------------------------------------|
| Name                      | ✅         |                                                  |
| World/Data Center         | ✅         |                                                  |
| Race/Subrace/Gender       | ✅         |                                                  |
| City-state                | ✅         |                                                  |
| Nameday                   | ✅         |                                                  |
| Guardian                  | ✅         |                                                  |
| Portrait/Full-body Images | ✅         | These are the images displayed on the Lodestone. |

Currently, more types of data is planned to be supported in the future.

## License

This project is licensed under the [GNU Affero General Public License 3](LICENSE).