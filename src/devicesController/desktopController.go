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

var previousSetMonitorBrightness = 1
var setMonitorBrightness = 0
var previousSetKbdBrightness = 0
var setKbdBrightness = 0
var workDesktopBrightnessCtrlPQ = utils.NewPriorityQueue()

// Pings desktop status every <frequency> secoends
// Updates brightness control pq
func UpdateSystemStatus(wg *sync.WaitGroup, systemAddress string, frequency int) {
	pqElementName := "offline"
	for {
		_, err := utils.SearchPQElement(workDesktopBrightnessCtrlPQ, pqElementName)
		isOnline := checkIfSystemIsOnline(systemAddress)
		// If the system is online and the pq has the offline element
		if isOnline && err == nil {
			utils.RemovePQElement(&workDesktopBrightnessCtrlPQ, pqElementName)
			log.Println("Work Desktop is online")
		} else if !isOnline && err != nil {
			utils.InsertPQElement(&workDesktopBrightnessCtrlPQ, *utils.NewPQElement(1, pqElementName))
			log.Println("Work Desktop is offline")
		}
		time.Sleep(time.Duration(frequency) * time.Second)
	}
	defer wg.Done()
}

// Tests if a system has rpc by checking for a daemon handling tcp port 135
// My best solution to check for a Windows system
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
		log.Panicf("Could not create new pinger. Err: %s", err)
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
			result += "online "
			if checksIfSystemHasRPC(v) {
				result += "and running Windows"
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
	case sensorBrightness >= 300:
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
	if sensorBrightness >= 65 {
		setMonitorBrightness = int(math.Round(float64(sensorBrightness) * coef))
	} else {
		setMonitorBrightness = 0
	}

	// Only send command if previous set value was different
	if previousSetMonitorBrightness != setMonitorBrightness {
		laptopBrightness := (setMonitorBrightness * maxBrightnessLaptop) / 100
		if laptopBrightness == 0 {
			laptopBrightness = 1000
		}

		monBrightStr := strconv.Itoa(setMonitorBrightness)
		laptopBrightStr := strconv.Itoa(laptopBrightness)

		log.Printf("Sending brightness command %d, laptop = %d", setMonitorBrightness, laptopBrightness)
		utils.Execute("ssh", "thinkpadx1-extreme",
			"ddcutil --bus 19 setvcp 10 "+monBrightStr+" & echo "+laptopBrightStr+" > /sys/class/backlight/intel_backlight/brightness")

		previousSetMonitorBrightness = setMonitorBrightness
	}
}
