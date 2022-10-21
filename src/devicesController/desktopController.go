package devicesController

import (
	"log"
	"os/exec"
	"strconv"
)

var previousSetMonitorBrightness = 0
var setMonitorBrightness = 0

func setDesktopBrightness(brightness int) {
	brightStr := strconv.Itoa(brightness)
	execute("ssh", "desktop", "sudo ddcutil --bus 6 setvcp 10 "+brightStr)
}


func ControlDesktopBrightness(sensorBrightness int) {
	// Maybe some linear regression stuff would be cool
	switch {
	case sensorBrightness >= 800:
		setMonitorBrightness = 100
	case sensorBrightness < 800 && sensorBrightness >= 600:
		setMonitorBrightness = 80
	case sensorBrightness < 600 && sensorBrightness >= 500:
		setMonitorBrightness = 60
	case sensorBrightness < 500 && sensorBrightness >= 400:
		setMonitorBrightness = 50
	case sensorBrightness < 400 && sensorBrightness >= 300:
		setMonitorBrightness = 30
	case sensorBrightness < 300 && sensorBrightness >= 200:
		setMonitorBrightness = 20
	case sensorBrightness < 200:
		setMonitorBrightness = 0
	}

	// Only send command if previous set value was different
	if previousSetMonitorBrightness != setMonitorBrightness {
		log.Printf("Sending brightness command %d", setMonitorBrightness)
		setDesktopBrightness(setMonitorBrightness)
		previousSetMonitorBrightness = setMonitorBrightness
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
