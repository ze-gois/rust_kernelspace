# kernelspace

[![crates.io](https://img.shields.io/crates/v/kernelspace.svg)](https://crates.io/crates/kernelspace)
[![docs.rs](https://docs.rs/kernelspace/badge.svg)](https://docs.rs/kernelspace)

Kernel-facing `no_std` experiments and abstractions for the [userspace.party](https://userspace.party) ecosystem.

## Role

`kernelspace` is the family member for code that crosses toward kernel-level execution and kernel symbols while reusing the lower-level primitives exposed by `ample` and `userspace`.

The current source tree includes:

- a `no_std` library surface;
- kernel module/symbol work under `modules::ksym`;
- a freestanding binary entry path and startup assembly;
- custom linker/build integration.

This crate is intentionally early-stage: the boundary and public API are still being defined as the bare-metal/kernel side of the ecosystem matures.

## Use

```bash
cargo add kernelspace
```

Registry dependencies are used by the standalone repository. Inside `userspace_hub`, those dependencies are patched to the pinned local submodules.

## Ecosystem

- Ecosystem: https://userspace.party
- Crate homepage: https://userspace.party/kernelspace
- API documentation: https://docs.rs/kernelspace
- crates.io: https://crates.io/crates/kernelspace
- Source: https://github.com/ze-gois/rust_kernelspace
- Workspace hub: https://github.com/ze-gois/rust_userspace_hub

## Status

Experimental systems software. Expect substantial API and architecture changes.

## License

See [LICENSE](LICENSE).
