# Simple "carrot" contract for ckb

This contract aims to reveal some problems with a simple Rust ckb contract. 

- Some compilation options will ruin the binary.
- Contract does not work as expected.

## Overview

To slim the size of contract binary in Rust, I use two groups of compilation optimization options. While one group of options will cause failure of contract execution.

### What does this contract (should) do?

The contract examines the output data. if data starts with bytes “carrot”, the contract will immediately return a non-zero value, and the validation fails.

```
2020-05-14 18:45:24.766 +08:00 GlobalRuntime-3 DEBUG ckb-script  script group: Byte32(0x17cdd44f05902b124ba1f1d56ea5f2b89bc9d72b26dd60639d3fff43129d9922) DEBUG OUTPUT: Let's see if you are carrying carrots.
2020-05-14 18:45:24.767 +08:00 GlobalRuntime-3 DEBUG ckb-script  script group: Byte32(0x17cdd44f05902b124ba1f1d56ea5f2b89bc9d72b26dd60639d3fff43129d9922) DEBUG OUTPUT: No!!! You have a carrot!
```

### Binary Size 

During compilation, two groups of options are used with different combination. **Above all, the `--release` flag is always used.**

The first group of options is write in *Cargo.toml*:

```toml
[profile.release]
lto = true
opt-level = 'z'
codegen-units = 1
```

Command for compilation:

```shell
cargo build --release
```

The other is used as env arg with `cargo build`:

```shell
RUSTFLAGS='-C link-arg=-s'

# Usage
RUSTFLAGS='-C link-arg=-s' cargo build --release
```

| Use `[profile.release]` | Use `RUSTFLAGS` | Binary Size |
| ----------------------- | --------------- | ----------- |
| ✗                       | ✗               | 891408      |
| ✓                       | ✗               | 145816      |
| ✗                       | ✓               | 29352       |
| ✓                       | ✓               | 25176       |

### What happens?

#### All the contract binary are patched with [`ckb-binary-patcher`](https://github.com/xxuejie/ckb-binary-patcher).

#### `[profile.release]` will mess up the binary

When I compile the contract using the option group `[profile.release]` (with `RUSTFLAGS`), the binary breaks down. Any call to the broken contract results to the below vm error:

```
Internal: VM(InvalidEcall(90112))
```

#### “carrots” will pass the examination

If I fill the output data field with byte string `carrot`, the contract will complain as expected. However, when output data is `carrots`, the contract will not complain.