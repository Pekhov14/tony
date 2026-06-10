package main

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
)

const (
	reset = "\x1b[0m"
	green = "\x1b[32m"
	cyan  = "\x1b[36m"
)

const (
	tonyfile = "Tonyfile.json"
)

func main() {
	if err := run(); err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}
}

func run() error {
	if len(os.Args) < 2 {
		return fmt.Errorf("usage: tony <command>")
	}

	arg := os.Args[1]

	commands, err := loadCommands(tonyfile)
	if err != nil {
		return err
	}

	command, ok := commands[arg]
	if !ok {
		return fmt.Errorf("unknown command: %s", arg)
	}

	fmt.Println(color(green, ">") + " " + color(cyan, command))

	return execute(command)
}

func color(code, text string) string {
	return code + text + reset
}

func loadCommands(path string) (map[string]string, error) {
	file, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	var commands map[string]string
	if err := json.Unmarshal(file, &commands); err != nil {
		return nil, err
	}

	return commands, nil
}

func execute(command string) error {
	result, err := exec.Command("sh", "-c", command).CombinedOutput()
	fmt.Print(string(result))
	if err != nil {
		return err
	}
	return nil
}
