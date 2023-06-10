package devicesController

import (
	"example.com/utils"
	"github.com/go-ping/ping"
	"log"
	"math"
	"net"
	"os"
	"strconv"
	"sync"
	"time"
)

var setLaptopBrightness = 0
var previousSetMonitorBrightness = 1
var setMonitorBrightness = 0
var previousSetKbdBrightness = 0
var setKbdBrightness = 0
var workDesktopBrightnessCtrlPQ = utils.NewPriorityQueue()
var isDesktop1online = false
var isDesktop2online = false
var isDesktop3online = false

// Pings for desktop 3 status every <frequency> seconds
// Updates global isDesktop3online variable
func UpdateDesktop3Status(wg *sync.WaitGroup, brightness int, frequency float32) {
	pqElementName := "offline"
	for {
		_, err := utils.SearchPQElement(workDesktopBrightnessCtrlPQ, pqElementName)
		isOnline := checkIfSystemIsOnline(os.Getenv("SYSTEM_3_ADDRESS"))
		// Log if desktop 2 changes status
		if isOnline && !isDesktop3online {
			log.Println("Desktop 3 is online")
		} else if !isOnline && isDesktop3online {
			log.Println("Desktop 3 is offline")
		}

		isDesktop3online = isOnline
		time.Sleep(time.Duration(frequency) * time.Second)

		// If the system is online and the pq has the offline element
		if isDesktop3online && err == nil {
			utils.RemovePQElement(&workDesktopBrightnessCtrlPQ, pqElementName)
			// update brightness control when changing status
			ControlKbdBacklightLaptop(brightness)
			ControlWorkDesktopBrightness(brightness)
		} else if !isDesktop3online && err != nil {
			utils.InsertPQElement(&workDesktopBrightnessCtrlPQ, *utils.NewPQElement(2, pqElementName))
		}
		time.Sleep(time.Duration(frequency) * time.Second)
	}
	defer wg.Done()
}

// Pings for desktop 2 status every <frequency> seconds
// Updates global isDesktop2online variable
func UpdateDesktop2Status(wg *sync.WaitGroup, frequency float32) {
	for {
		isOnline := checkIfSystemIsOnline(os.Getenv("SYSTEM_2_ADDRESS"))
		// Log if desktop 2 changes status
		if isOnline && !isDesktop2online {
			log.Println("Desktop 2 is online")
		} else if !isOnline && isDesktop2online {
			log.Println("Desktop 2 is offline")
		}
		isDesktop2online = isOnline
		time.Sleep(time.Duration(frequency) * time.Second)
	}
	defer wg.Done()
}

// Pings for desktop 1 status every <frequency> seconds
// Updates global isDesktop1online variable
// Updates brightness control priority queue with (offline, 1) element
// Sends brightness control command when changing offline to online
func UpdateDesktop1Status(wg *sync.WaitGroup, brightness int, frequency float32) {
	pqElementName := "offline"
	for {
		_, err := utils.SearchPQElement(workDesktopBrightnessCtrlPQ, pqElementName)
		isOnline := checkIfSystemIsOnline(os.Getenv("SYSTEM_1_ADDRESS"))

		// Log if desktop 2 changes status
		if isOnline && !isDesktop1online {
			log.Println("Desktop 1 is online")
		} else if !isOnline && isDesktop1online {
			log.Println("Desktop 1 is offline")
		}

		isDesktop1online = isOnline

		// If the system is online and the pq has the offline element
		if isDesktop1online && err == nil {
			utils.RemovePQElement(&workDesktopBrightnessCtrlPQ, pqElementName)
			// update brightness control when changing status
			ControlKbdBacklightLaptop(brightness)
			ControlWorkDesktopBrightness(brightness)
		} else if !isDesktop1online && err != nil {
			utils.InsertPQElement(&workDesktopBrightnessCtrlPQ, *utils.NewPQElement(1, pqElementName))
		}
		time.Sleep(time.Duration(frequency) * time.Second)
	}
	defer wg.Done()
}

// My best solution to check if a system is running Windows
// Test if a system has rpc by checking for a daemon handling tcp port 135
func checksIfSystemHasRPC(address string) bool {
	conn, err := net.DialTimeout("tcp", address+":135", 1*time.Second)
	if err != nil {
		return false
	} else {
		conn.Close()
		return true
	}
}

// Tests if a desktop is online by icmp packet
// System is offline if packet loss is 100%
func checkIfSystemIsOnline(address string) bool {
	pinger, err := ping.NewPinger(address)
	if err != nil {
		log.Printf("Could not create new pinger. Err: %s", err)
	}
	pinger.Count = 1
	pinger.Timeout = 1 * time.Second
	err = pinger.Run() // Blocks until finished.
	if err != nil {
		log.Panicf("Could not run pinger. Err: %s", err)
	}

	stats := pinger.Statistics().PacketLoss

	if stats == 100 {
		return false
	} else {
		return true
	}
}

// Tests if a system has ssh by checking for a daemon handling tcp port 22
func checkIfSystemHasSSH(address string) bool {
	conn, err := net.DialTimeout("tcp", address+":22", 1*time.Second)
	if err != nil {
		return false
	} else {
		conn.Close()
		return true
	}
}

// Checks if desktop is online and what OS is running
func GetSystemStatus() string {
	systemAddresses := [...]string{os.Getenv("SYSTEM_1_ADDRESS"), os.Getenv("SYSTEM_2_ADDRESS")}
	result := ""
	for i, v := range systemAddresses {
		result += "System " + strconv.Itoa(i+1) + " is "

		// Logic tree to find running OS and SSH
		if checkIfSystemIsOnline(v) {
			result += "online"
			if checksIfSystemHasRPC(v) {
				result += " and running Windows"
			} else {
				result += ", running Linux and SSH is "
				if checkIfSystemHasSSH(v) {
					result += "up"
				} else {
					result += "down"
				}
			}
		} else {
			result += "offline"
		}
		result += "\n"
	}

	return result
}

// Checks if "disableBrightnessAutoControl" key is in the pq.
// If true, removes
func EnableAutomaticBrightnessControl() string {
	_, err := utils.SearchPQElement(workDesktopBrightnessCtrlPQ, "disableBrightnessAutoControl")
	if err == nil {
		utils.RemovePQElement(&workDesktopBrightnessCtrlPQ, "disableBrightnessAutoControl")
		return "Enabled brightness auto control"
	} else {
		return "Brightness auto control is already enabled!"
	}
}

func DisableAutomaticBrightnessControl() string {
	_, err := utils.SearchPQElement(workDesktopBrightnessCtrlPQ, "disableBrightnessAutoControl")
	if err == nil {
		return "Brightness auto control is already disabled!"
	} else {
		utils.InsertPQElement(&workDesktopBrightnessCtrlPQ, *utils.NewPQElement(2, "disableBrightnessAutoControl"))
		return "Disabled brightness auto control"
	}
}

func ControlKbdBacklightLaptop(sensorBrightness int) {
	if len(workDesktopBrightnessCtrlPQ) > 0 {
		return
	}

	switch {
	case sensorBrightness >= 250:
		setKbdBrightness = 0
	case sensorBrightness < 250 && sensorBrightness >= 100:
		setKbdBrightness = 2
	case sensorBrightness < 100:
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

func ControlWorkDesktopBrightness(sensorBrightness int) {
	if len(workDesktopBrightnessCtrlPQ) > 0 {
		return
	}

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

	// Set minumum brightness values
	if sensorBrightness >= 65 {
		setMonitorBrightness = int(math.Round(float64(sensorBrightness) * coef))
		setLaptopBrightness = (setMonitorBrightness * maxBrightnessLaptop) / 100
	} else {
		setMonitorBrightness = 0
		setLaptopBrightness = 1000
	}

	// Check if values have overflowed
	if setMonitorBrightness > 100 || setLaptopBrightness > maxBrightnessLaptop {
		setMonitorBrightness = 100
		setLaptopBrightness = maxBrightnessLaptop
	}

	// Send command only if previous set value was different by two
	// Intention is to avoid frequent brightness updates
	if math.Abs(float64(previousSetMonitorBrightness-setMonitorBrightness)) > 2 {
		monBrightStr := strconv.Itoa(setMonitorBrightness)
		laptopBrightStr := strconv.Itoa(setLaptopBrightness)

		log.Printf("Sending brightness command %d, laptop = %d", setMonitorBrightness, setLaptopBrightness)

		// go utils.Execute("ssh", "thinkpadx1-extreme", "ddcutil --bus 13 setvcp 10 "+monBrightStr)
		go utils.Execute("ssh", "thinkpadx1-extreme", "echo "+laptopBrightStr+" > /sys/class/backlight/intel_backlight/brightness & ddcutil --bus 14 setvcp 10 "+monBrightStr)

		previousSetMonitorBrightness = setMonitorBrightness
	}
}
