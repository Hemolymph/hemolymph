# Hemolymph
Hemolymph is a search engine for the card game Bloodless. For features, see [Hemoglobin](https://github.com/Lilith-In-Starlight/hemoglobin)'s search features.

## Building
You need to install [Trunk](https://trunkrs.dev/).

Once you have it installed and accessible as a command line tool, clone this repository wherever you want it.

Run `make` to build the frontend. This version of the frontend will connect to the official Hemolymph API. Run `make debug` for one connected to a server running locally in the `8080` port.

Then, `cargo run` the server.
