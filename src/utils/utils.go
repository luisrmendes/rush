package utils

import (
	"bufio"
	"context"
	"fmt"
	"github.com/joho/godotenv"
	"log"
	"os"
	"os/exec"
	"time"
)

func ParseEnvVars() {
	testEnvVar := os.Getenv("ESP8266_ADDRESS_PORT")

	if testEnvVar == "" {
		log.Printf("ESP8266_ADDRESS_PORT not found in env vars, checking .env")
		err := godotenv.Load(".env")
		if err != nil {
			log.Panicf("Cannot load .env file: %s", err)
		}
	}
}

// Executes terminal calls
// Returns the output of the command
// Handles errors outputted by the command call
func Execute(name string, args ...string) string {
	ctx, cancel := context.WithTimeout(context.Background(), 4*time.Second)
	defer cancel()

	cmd := exec.CommandContext(ctx, name, args...)
	err := cmd.Run()

	if err != nil {
		log.Printf("Command %s returned error: %s", name, cmd.Stderr)
		return ""
	}

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		fmt.Println(err)
	}

	// Create a bufio.Reader from cmd.Stdout
	r := bufio.NewReader(stdout)

	// Read the output of cmd.Stdout into a string
	output, err := r.ReadString('\n')
	if err != nil {
		fmt.Println(err)
	}

	return output
}
