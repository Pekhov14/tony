package main

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	s "tony/internal/style"
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

	fmt.Print(s.Color(s.Green, "> "))
	fmt.Println(s.Color(s.Cyan, command))

	return execute(command)
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
