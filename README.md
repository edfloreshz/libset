# libset
A configuration file management library for Rust applications.

## Add the library
Run `cargo add libset` or add it to `Cargo.toml`:

```toml
[dependencies]
libset = "0.1"
```

## Usage

Start by creating a new `Config` object:
```rust
let config = Config::new("org.example.Demo", 1, None)?;
```
Provide an application name, a version and optionally a prefix, then, a new directory will be added to your filesystem, this is where all the created files will be stored in.

### Write a file.
```rust
let config = Config::new("org.example.Demo", 1, None)?;
config.set_json("colors", json!({ "accent": "#7a7af9" }))?;
```
> This wil store the file here: `$HOME/.config/org.example.Demo/v1/colors.json`

### Get a file.
```rust
#[derive(Debug, Serialize, Deserialize)]
struct Colors { accent: String }
let settings: Colors = config.get_json("colors")?;
```

Check out the examples!

### Scopes
A scope is just a simple sub-directory stored inside your application's config directory, all subsequent files will be stored within that scope.
```rust
let config = Config::new("org.example.Demo", 1, Some("appearance"))?;
config.set_json("colors", json!({ "accent": "#7a7af9" }))?;
```
> This wil store the file here: `$HOME/.config/org.example.Demo/v1/appearance/colors.json`

## Available features
- `json` - Enables json support, enabled by default.
- `toml` - Enables toml support.
- `ron`  - Enables ron support.

Depending on which features you enable, you will get setter and getters methods suffixed by the file type.

## Proposals

If you have a proposal for a new feature, open a new [issue](https://github.com/edfloreshz/libset/issues).
