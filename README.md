# Tony

<p align="center">
<img src="_tony_.webp" alt="tony" width="320">
</p>

`tony` is a blazing-fast, developer-friendly task runner written in Rust. Inspired by `make`, it helps you organize and run project commands with clean aliases and zero friction.

---

## Why `tony`?

Instead of memorizing and typing long, cumbersome commands:

```sh
docker build -t my-app . && docker push my-app:latest
```

You can define short, memorable aliases in a tonyfile.toml:

```toml
[tasks]
build = "docker build -t my-app ."
push  = "docker push my-app:latest"
```

And run them instantly:
```sh
tony build
```
---

### Install from GitHub Releases

1. Download the archive for your platform from the [releases page](https://github.com/Pekhov14/tony/releases).
2. Extract it:

```sh
tar -xzf tony_<version>_<platform>.tar.gz
```

3. Move the binary to a directory in your `PATH`, for example:

```sh
sudo mv tony /usr/local/bin/tony
```

### macOS note

On macOS, the binary may be blocked on first launch because it is not code-signed yet.
If that happens, remove the quarantine attribute and try again:

```sh
xattr -d com.apple.quarantine /usr/local/bin/tony
```
## Usage

`tony` automatically looks for `tonyfile.toml` in the current working directory.

```sh
tony <command>         # Run a specific task
tony <command> --quiet # Run quietly (suppress command echoing)
tony <command> -q      # Run quietly with short syntax (suppress command echoing)
```

## Uninstallation
If you ever need to remove tony:

```bash
sudo rm /usr/local/bin/tony
```

Example `tonyfile.toml`:

```toml
[tasks]
build = "docker build -t my-app ."
push  = "docker push my-app:latest"
```
