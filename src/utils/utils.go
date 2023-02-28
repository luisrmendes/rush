package utils

import (
	"github.com/joho/godotenv"
	"log"
	"os"
	"os/exec"
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
// TODO: why do I need this?
func Execute(name string, args ...string) string {
	out, err := exec.Command(name, args...).Output()
	if err != nil {
		log.Printf("Command %s %s returned error: %s", name, args, err)
	}
	output := string(out[:])

	return output
}
