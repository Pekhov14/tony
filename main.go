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

	tonyfile = "Tonyfile.json"
)

func color(code, text string) string {
	return code + text + reset
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("usage: tony <command>")
		os.Exit(1)
	}

	arg := os.Args[1]

	file, err := os.ReadFile(tonyfile)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	var commands map[string]string
	if err = json.Unmarshal(file, &commands); err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	command, ok := commands[arg]
	if !ok {
		fmt.Println("Unknown command:", arg)
		os.Exit(1)
	}


	fmt.Println(color(green, ">") + " " + color(cyan, command))

	result, err := exec.Command("sh", "-c", command).CombinedOutput()

	fmt.Print(string(result))

	if err != nil {
		os.Exit(1)
	}
}
