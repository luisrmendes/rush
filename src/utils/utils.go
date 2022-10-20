package utils

import (
	"log"
	"os/exec"
	"strconv"
)

var previousBrightness = 0

func setDesktopBrightness(brightness int) {
	brightStr := strconv.Itoa(brightness)
	execute("ssh", "desktop", "sudo ddcutil --bus 6 setvcp 10 "+brightStr)
}

// Maybe some linear regression stuff would be cool
func ControlDesktopBrightness(brightness int) {
	if absDiff(previousBrightness, brightness) > 100 {
		log.Println("Sending brightness command")
		switch {
		case brightness >= 800:
			setDesktopBrightness(100)
		case brightness < 800 && brightness >= 600:
			setDesktopBrightness(80)
		case brightness < 600 && brightness >= 500:
			setDesktopBrightness(60)
		case brightness < 500 && brightness >= 400:
			setDesktopBrightness(50)
		case brightness < 400 && brightness >= 300:
			setDesktopBrightness(30)
		case brightness < 300 && brightness >= 200:
			setDesktopBrightness(20)
		case brightness < 200:
			setDesktopBrightness(0)
		}
	}
	previousBrightness = brightness
}

func execute(name string, args ...string) string {
	out, err := exec.Command(name, args...).Output()
	if err != nil {
		log.Fatalf("Command %s gave error %s", name, err)
	}
	output := string(out[:])

	return output
}

func absDiff(a, b int) int {
	if a < b {
		return b - a
	}
	return a - b
}
