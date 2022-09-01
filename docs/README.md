<style>
  ul li:not(:last-child) { margin-bottom: 0.4em; }
</style>

- [Overview](#overview)
- [Installation](#installation)
- [Scripts](#scripts)
- [Executable Scripts](#executable-scripts)
- [Expressions](#expressions)
- [Filters](#filters)
- [Environment Variables](#environment-variables)
- [Templates](#templates)
- [Troubleshooting](#troubleshooting)

## Overview

With `rust-script` Rust files and expressions can be executed just like a shell or Python script. Features include:

- Caching compiled artifacts for speed.
- Reading Cargo manifests embedded in Rust scripts.
- Supporting executable Rust scripts via Unix shebangs and Windows file associations.
- Using expressions as stream filters (*i.e.* for use in command pipelines).
- Running unit tests and benchmarks from scripts.
- Custom templates for command-line expressions and filters.

You can get an overview of the available options using the `--help` flag.

## Installation

Install or update `rust-script` using Cargo:

```sh
cargo install rust-script
```

Rust 1.47 or later is required.

## Scripts

The primary use for `rust-script` is for running Rust source files as scripts. For example:

```sh
$ echo 'println!("Hello, World!");' > hello.rs
$ rust-script hello.rs
Hello, World!
```

Under the hood, a Cargo project will be generated and built (with the Cargo output hidden unless compilation fails or the `-o`/`--cargo-output` option is used). The first invocation of the script will be slower as the script is compiled - subsequent invocations of unmodified scripts will be fast as the built executable is cached.

As seen from the above example, using a `fn main() {}` function is not required. If not present, the script file will be wrapped in a `fn main() { ... }` block.

`rust-script` will look for embedded dependency and manifest information in the script as shown by the below `now.rs` variants:

```rust
#!/usr/bin/env rust-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! time = "0.1.25"
//! ```
fn main() {
    println!("{}", time::now().rfc822z());
}
```

Useful command-line arguments:

- `--bench`: Compile and run benchmarks.  Requires a nightly toolchain.
- `--debug`: Build a debug executable, not an optimised one.
- `--features <features>`: Cargo features to pass when building and running.
- `--force`: Force the script to be rebuilt.  Useful if you want to force a recompile with a different toolchain.
- `--test`: Compile and run tests.

## Executable Scripts

On Unix systems, you can use `#!/usr/bin/env rust-script` as a shebang line in a Rust script.  This will allow you to execute a script files (which don't need to have the `.rs` file extension) directly.

If you are using Windows, you can associate the `.ers` extension (executable Rust - a renamed `.rs` file) with `rust-script`.  This allows you to execute Rust scripts simply by naming them like any other executable or script.

This can be done using the `rust-script --install-file-association` command. Uninstall the file association with `rust-script --uninstall-file-association`.

If you want to make a script usable across platforms, use *both* a shebang line *and* give the file a `.ers` file extension.

## Expressions

Using the `-e`/`--expr` option a Rust expression can be evaluated directly, with dependencies (if any) added using `-d`/`--dep`:

```sh
$ rust-script -e '1+2'
3
$ rust-script --dep time --expr "time::OffsetDateTime::now_utc().format(time::Format::Rfc3339).to_string()"`
"2020-10-28T11:42:10+00:00"
$ # Use a specific version of the time crate (instead of default latest):
$ rust-script --dep time=0.1.38 -e "time::now().rfc822z().to_string()"
"2020-10-28T11:42:10+00:00"
```

The code given is embedded into a block expression, evaluated, and printed out using the `Debug` formatter (*i.e.* `{:?}`).

## Filters

TBD

Note that, like with expressions, you can specify a custom template for stream filters.

## Environment Variables

The following environment variables are provided to scripts by `rust-script`:

- `RUST_SCRIPT_BASE_PATH`: the base path used by `rust-script` to resolve relative dependency paths.  Note that this is *not* necessarily the same as either the working directory, or the directory in which the script is being compiled.

- `RUST_SCRIPT_PKG_NAME`: the generated package name of the script.

- `RUST_SCRIPT_SAFE_NAME`: the file name of the script (sans file extension) being run.  For scripts, this is derived from the script's filename.  May also be `"expr"` or `"loop"` for those invocations.

- `RUST_SCRIPT_PATH`: absolute path to the script being run, assuming one exists.  Set to the empty string for expressions.

## Templates

You can use templates to avoid having to re-specify common code and dependencies.  You can find out the directory where templates are store and view a list of your templates by running `rust-script --list-templates`.

Templates are Rust source files with two placeholders: `#{prelude}` for the auto-generated prelude (which should be placed at the top of the template), and `#{script}` for the contents of the script itself.

For example, a minimal expression template that adds a dependency and imports some additional symbols might be:

```rust
//! ```cargo
//! [dependencies]
//! itertools="0.6.2"
//! ```
#![allow(unused_imports)]
#{prelude}
use std::io::prelude::*;
use std::mem;
use itertools::Itertools;

fn main() {
    let result = {
        #{script}
    };
    println!("{:?}", result);
}
```

If stored in the templates folder as `grabbag.rs`, you can use it by passing the name `grabbag` via the `--template` option, like so:

```sh
$ rust-script -t grabbag -e "mem::size_of::<Box<Read>>()"
16
```

In addition, there are three built-in templates: `expr`, `loop`, and `loop-count`.  These are used for the `--expr`, `--loop`, and `--loop --count` invocation forms.  They can be overridden by placing templates with the same name in the template folder.

## Troubleshooting

Please report all issues on [the GitHub issue tracker](https://github.com/fornwall/rust-script/issues).

If relevant, run with the `RUST_LOG=rust_script=trace` environment variable set to see verbose log output and attach that output to an issue.
