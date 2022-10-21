package utils

import (
	"log"
	"os/exec"
)

// Executes terminal calls
// Returns the output of the command
// Handles errors outputted by the command call
func Execute(name string, args ...string) string {
	out, err := exec.Command(name, args...).Output()
	if err != nil {
		log.Printf("Command %s returned error: %s", name, err)
	}
	output := string(out[:])

	return output
}