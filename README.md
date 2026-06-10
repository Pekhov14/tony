# tony

![tony](tony.webp)

`tony` is a small CLI tool inspired by `make` and similar task runners.
It reads commands from `Tonyfile.json` and runs them by alias.

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

`tony` looks for `Tonyfile.json` in the current working directory.

```sh
tony <command>
```

Example `Tonyfile.json`:

```json
{
  "docker-build": "docker build -t my-app .",
  "frontend-build": "vite build",
  "test": "go test ./...",
  "lint": "golangci-lint run",
  "dev": "docker compose up"
}
```

Then run:

```sh
tony docker-build
tony frontend-build
tony test
```

## Run locally

If you built the binary yourself and it is still in the current directory:

```sh
./tony <command>
```

## Output

Before running a command, `tony` prints it with simple color styling:

```sh
tony frontend-build
> vite build
vite v7.0.0 building for production...
...
```

The command itself is highlighted, while the actual command output is printed as-is.

## How it works

1. `tony` reads `Tonyfile.json`
2. finds the requested command by name
3. runs it through `sh -c`
4. prints the command output to the terminal

## Current limitations

- `Tonyfile.json` must exist in the current working directory
- commands are executed through `sh -c`
- the current release builds target macOS and Linux
- the config format is currently just `map[string]string`

## Build from source

```sh
go build -o tony
```

## License

MIT
