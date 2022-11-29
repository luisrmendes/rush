package devicesController

import (
	"example.com/utils"
	"log"
	"strconv"
)

var previousSetMonitorBrightness = 0
var setMonitorBrightness = 0

func ControlDesktopBrightness(sensorBrightness int) {
	maxBrightnessLaptop := 19393

	// TODO: Maybe some linear regression stuff would be cool
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
		setMonitorBrightness = 1
	}

	// Only send command if previous set value was different
	if previousSetMonitorBrightness != setMonitorBrightness {
		laptopBrightness := (setMonitorBrightness * maxBrightnessLaptop) / 100
		monBrightStr := strconv.Itoa(setMonitorBrightness)
		laptopBrightStr := strconv.Itoa(laptopBrightness)
		log.Printf("Sending brightness command %d, laptop = %d", setMonitorBrightness, laptopBrightness)
		utils.Execute("ssh", "thinkpadx1-extreme", "ddcutil --bus 14 setvcp 10 "+monBrightStr+" & echo "+laptopBrightStr+" > /sys/class/backlight/intel_backlight/brightness")
		previousSetMonitorBrightness = setMonitorBrightness
	}
}
