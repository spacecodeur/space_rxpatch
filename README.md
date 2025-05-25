# space_rxpatch

**space_rxpatch** is a command-line tool for surgically editing any kind of text file using the full power of regular expressions (regex).  
It allows precise insertion or replacement of text at positions matched by regex patterns — even across multiple lines.

## Why this project?

Working with large or complex configuration files, source code, or data files often requires automating precise text manipulations.  
Traditional tools like `sed` or `awk` can be limiting when:

- You need access to **regex capture groups**,
- You need to **insert text inline or in blocks**,
- You want to know the **exact line and column** of matches,
- You want a solution that's **portable**, **modern**, and **easy to extend**.

**space_rxpatch** is designed to solve these cases cleanly with Rust's reliability and performance.

## What can it do?

- Match patterns using **regex with capture groups**
- Perform **inline replacements** (e.g., replacing content between `{...}`)
- Insert **multi-line blocks** precisely between matched anchors
- Track and print **line/column positions** of matches
- Work on any UTF-8 text file

## Installation

(Coming soon — for now, clone the repo and build with Cargo)

```bash
git clone https://github.com/spacecodeur/space_rxpatch.git
cd space_rxpatch
cargo build --release
```

## Usage

coming soon...

### Examples

coming soon...

## License

MIT — free to use, modify, and distribute.