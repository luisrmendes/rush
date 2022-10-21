package devicesController

import (
	"log"
	"os/exec"
	"strconv"
)

var previousBrightness = 0
var brightnessChangeThreshold = 20


func setDesktopBrightness(brightness int) {
	brightStr := strconv.Itoa(brightness)
	execute("ssh", "desktop", "sudo ddcutil --bus 6 setvcp 10 "+brightStr)
}

// Maybe some linear regression stuff would be cool
func ControlDesktopBrightness(brightness int) {
	setBrightness := 100
	// Only send command if previous set value was significantly different
	if absDiff(previousBrightness, brightness) > brightnessChangeThreshold {
		switch {
		case brightness >= 800:
			setBrightness = 100
		case brightness < 800 && brightness >= 600:
			setBrightness = 80
		case brightness < 600 && brightness >= 500:
			setBrightness = 60
		case brightness < 500 && brightness >= 400:
			setBrightness = 50
		case brightness < 400 && brightness >= 300:
			setBrightness = 30
		case brightness < 300 && brightness >= 200:
			setBrightness = 20
		case brightness < 200:
			setBrightness = 0
		}
		log.Printf("Sending brightness command %d", setBrightness)
		setDesktopBrightness(setBrightness)
		previousBrightness = brightness
	}
}

// Execute bash commands, handles errors
func execute(name string, args ...string) string {
	out, err := exec.Command(name, args...).Output()
	if err != nil {
		log.Printf("Command %s gave error %s", name, err)
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
