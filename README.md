# tony

![tony](tony.webp)

`tony` is a small CLI tool inspired by `make` and similar task runners.
It reads commands from `Tonyfile.json` and runs them by alias.

## Why

Instead of typing long commands like:

```sh
docker build -t my-app .
```

or documentation building, testing, linting, or running the dev server.

Need to create a `Tonyfile.json` file first, then you can define short aliases and run them with `tony`.

## Example `Tonyfile.json`

```json
{
  "docker-build": "docker build -t my-app .",
  "frontend-build": "vite build",
  "test": "go test ./...",
  "lint": "golangci-lint run",
  "dev": "docker compose up"
}
```

## Usage

```sh
tony <command>
```

Then run:

```sh
tony docker-build
tony frontend-build
tony test
```

## Run locally

```sh
./tony <command>
```

Optional: make it available globally
Move the binary to a directory in your `PATH`, for example:

```sh
sudo mv tony /usr/local/bin/tony
```
Then you can run:

```sh
tony <command>
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
- the config format is currently just `map[string]string`

## License

MIT

## Build from source

```sh
go build -o tony
```
