# UnderanalyzerRS

This Rust crate provides safe bindings to the C# library
[Underanalyzer](https://www.nuget.org/packages/UnderminersTeam.Underanalyzer)
for the Rust library [LibGM](https://crates.io/crates/libgm).

Since this crate is only compilable with the .NET toolchain installed, it will
never be suitable for publishing on crates.io.

The plan is that Underanalyzer will be rewritten in pure Rust later,
when enough contributors are gathered.

## Usage

In your `Cargo.toml`:

```toml
[dependencies]
libgm = "..."  # whatever version
underanalyzer = { git = "https://github.com/BioTomateDE/UnderanalyzerRS" }
```

In your Rust code:

```rust
let data: GMData = libgm::parse_file("./data.win")?;
let ctx = GameContext::new(&data)?;
// This operation may take a few hundred milliseconds.
// You should therefore reuse this `GameContext` whenever possible.

let code: GMRef<GMCode> = data.codes.ref_by_name("gml_Script_attention_hackerz_no2")?;
let output: String = ctx.decompile(code, &data)?;
println!("{output}");
```

Please note that this `GameContext` struct may need to be updated (reconstructed)
when major parts of the data file are changed.

## Stability

This interaction between Rust and C# is probably pretty unstable.
(That's why I wanna rewrite Underanalyzer in Rust anyway.)

## Licence

This wrapper library is licensed under GPL-3.0.
The underlying library resposible for the decompilation,
Underanalyzer, is licensed under MPL-2.0.
