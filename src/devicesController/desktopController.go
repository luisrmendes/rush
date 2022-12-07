package devicesController

import (
	"example.com/utils"
	"log"
	"math"
	"strconv"
)

var previousSetMonitorBrightness = 0
var setMonitorBrightness = 1
var previousSetKbdBrightness = 0
var setKbdBrightness = 0

func ControlKbdBacklightLaptop(sensorBrightness int) {
	switch {
	case sensorBrightness >= 800:
		setKbdBrightness = 0
	case sensorBrightness < 300 && sensorBrightness >= 200:
		setKbdBrightness = 2
	case sensorBrightness < 200:
		setKbdBrightness = 1
	}

	// Only send command if previous set value was different
	if previousSetKbdBrightness != setKbdBrightness {
		setKbdBrightStr := strconv.Itoa(setKbdBrightness)
		log.Printf("Sending kbd brightness command %d", setKbdBrightness)
		utils.Execute("ssh", "thinkpadx1-extreme", "echo "+setKbdBrightStr+" > /sys/class/leds/tpacpi::kbd_backlight/brightness")
		previousSetKbdBrightness = setKbdBrightness
	}
}

func ControlDesktopBrightness(sensorBrightness int) {
	maxBrightnessLaptop := 19393

	// TODO: Maybe some linear regression stuff would be cool, increase granularity
	// switch {
	// case sensorBrightness >= 800:
	// 	setMonitorBrightness = 100
	// case sensorBrightness < 800 && sensorBrightness >= 600:
	// 	setMonitorBrightness = 80
	// case sensorBrightness < 600 && sensorBrightness >= 500:
	// 	setMonitorBrightness = 60
	// case sensorBrightness < 500 && sensorBrightness >= 350:
	// 	setMonitorBrightness = 50
	// case sensorBrightness < 350 && sensorBrightness >= 300:
	// 	setMonitorBrightness = 30
	// case sensorBrightness < 300 && sensorBrightness >= 220:
	// 	setMonitorBrightness = 20
	// case sensorBrightness < 220 && sensorBrightness >= 150:
	// 	setMonitorBrightness = 10
	// case sensorBrightness < 150:
	// 	setMonitorBrightness = 0
	// }

	var coef float64 = 0.14285714285714285
	if sensorBrightness >= 150 {
		setMonitorBrightness = int(math.Round(float64(sensorBrightness) * coef))
	} else {
		setMonitorBrightness = 0
	}

	// Only send command if previous set value was different
	if previousSetMonitorBrightness != setMonitorBrightness {
		laptopBrightness := (setMonitorBrightness * maxBrightnessLaptop) / 100
		if laptopBrightness == 0 {
			laptopBrightness = 100
		}

		monBrightStr := strconv.Itoa(setMonitorBrightness)
		laptopBrightStr := strconv.Itoa(laptopBrightness)

		log.Printf("Sending brightness command %d, laptop = %d", setMonitorBrightness, laptopBrightness)
		utils.Execute("ssh", "thinkpadx1-extreme", "ddcutil --bus 14 setvcp 10 "+monBrightStr+" & echo "+laptopBrightStr+" > /sys/class/backlight/intel_backlight/brightness")

		previousSetMonitorBrightness = setMonitorBrightness
	}
}
