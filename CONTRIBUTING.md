# Module structure, and how to contribute

The module structure here is unusual:
- `lib.rs` doesn't provide anything. And
- benchmark scaffolding is in [benches/shared/lib_benches.rs](benches/shared/lib_benches.rs).

If you use VS Code: [.vscode/settings.json](.vscode/settings.json) activates `fastrand` (currently
the only randomness generator).

See also
- [README.md](README.md), and
- [cami-rs/cami -> CONTRIBUTING.md](https://github.com/cami-rs/cami/blob/main/CONTRIBUTING.md).
