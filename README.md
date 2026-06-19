# tony

<p align="center">
<img src="_tony_.webp" alt="tony" width="320">
</p>

`tony` is a small CLI tool inspired by `make` and similar task runners.
It reads commands from `tonyfile.toml` and runs them by alias.

## Why

Instead of typing long commands like:

```sh
docker build -t my-app .
vite build
go test ./...
```

you can define short aliases in `Tonyfile.json` and run them with `tony`.

## Install from GitHub Releases

1. Download the archive for your platform from the [releases page](https://github.com/Pekhov14/tony/releases).
2. Extract it:

```sh
tar -xzf tony_<version>_<platform>.tar.gz
```

3. Move the binary to a directory in your `PATH`, for example:

```sh
sudo mv tony /usr/local/bin/tony
```

Then you can run:

```sh
tony <command>
```

### macOS note

On macOS, the binary may be blocked on first launch because it is not code-signed yet.
If that happens, remove the quarantine attribute and try again:

```sh
xattr -d com.apple.quarantine /usr/local/bin/tony
```

## Usage

`tony` looks for `tonyfile.toml` in the current working directory.

```sh
tony <command>
```

Example `tonyfile.toml`:

```toml
build = "docker build -t my-app ."
run = "cargo run pwd --quiet"
lint = "golangci-lint run"
dev = "docker compose up"
```

Then run:

```sh
tony build
tony run
```
