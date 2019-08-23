# Lister

Recursively shows the most-recently-modified files in the current directory. It
is roughly equivalent to:

    ls -t **/* | head -n NUM

...but is quite a bit faster.

It ignores hidden ("dot") files and does not print directory names (but does
descend into all directories). Unlike some Rust crates (ripgrep or fd), it
intentionally does not check gitignore-type files.

## Installation

    cargo install --git https://github.com/gabebw/rust-lister

## Usage

Show the most recent 10 files:

  lister

Show the most recent N files:

  lister N

Sample output in the rust-lister directory:

    README.md
    target/debug/incremental/lister-2hqk5y207glfy/s-ffaxwjbcuk-lnl10n.lock
    target/debug/incremental/lister-2hqk5y207glfy/s-ffaxwjbcuk-lnl10n-2vy24mx4eyp5t/query-cache.bin
    target/debug/incremental/lister-2hqk5y207glfy/s-ffaxwjbcuk-lnl10n-2vy24mx4eyp5t/dep-graph.bin
    target/debug/incremental/lister-2hqk5y207glfy/s-ffaxwjbcuk-lnl10n-2vy24mx4eyp5t/work-products.bin
    target/debug/deps/liblister-b28c1f3853cda82f.rmeta
    target/debug/deps/lister-b28c1f3853cda82f.d
    src/main.rs
    target/debug/incremental/lister-3bvkul16dfn78/s-ffaxvoxtmc-1dlids3.lock
    target/debug/deps/lister-7821e324beda4949.d
