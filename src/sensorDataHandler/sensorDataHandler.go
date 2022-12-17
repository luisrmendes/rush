package sensorDataHandler

import (
	"example.com/devicesController"
	"github.com/joho/godotenv"
	"log"
	"net"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"
)

type SensorData struct {
	brightness  int
	temperature int
	humidity    int
}

var handleSensorDataDelay time.Duration = 1000000000 // nanoseconds
var logSensorDataInterval time.Duration = 2 // minutes

// Looping function
// Reads sensor data from ESP8266 TCP server
// Handles if TCP server shutdowns
// Calls handleSensorData
func ReadSensorData(wg *sync.WaitGroup, sData *SensorData) {
	// Fetch environment variables
	esp8266_address_port := os.Getenv("ESP8266_ADDRESS_PORT")
	if esp8266_address_port == "" {
		log.Printf("ESP8266_ADDRESS_PORT not found in env vars, checking .env")
		err := godotenv.Load(".env")
		if err != nil {
			log.Panicf("Some error occured. Err: %s", err)
		}
	}
	esp8266_address_port = os.Getenv("ESP8266_ADDRESS_PORT")
	
	for {
		connection, err := net.Dial("tcp", esp8266_address_port)
		if err != nil {
			log.Println(err)
			continue
		} else {
			log.Println("Connected to " + esp8266_address_port)
		}
		for {
			buffer := make([]byte, 1024)
			mLen, err := connection.Read(buffer)
			if err != nil {
				log.Println("Error reading:", err.Error())
				log.Println("Disconnecting...")
				break
			}
			// Split by space character
			splitSensorData := strings.SplitAfter(string(buffer[:mLen]), " ")

			// remove last element of each split value (its a space!), convert to int
			sData.brightness, _ = strconv.Atoi(splitSensorData[1][:len(splitSensorData[1])-1])
			sData.temperature, _ = strconv.Atoi(splitSensorData[2][:len(splitSensorData[2])-1])
			sData.humidity, _ = strconv.Atoi(splitSensorData[3])
		}
	}
	defer wg.Done()
}

// Handle sensor data
func HandleSensorData(wg *sync.WaitGroup, sData *SensorData) {
	ticker := time.NewTicker(time.Minute * logSensorDataInterval)
	for {
		time.Sleep(handleSensorDataDelay)

		// Only handle data when data is being sent
		if sData.brightness == 0 {
			continue
		}

		// goroutine to log sensor data
		go func(sData *SensorData) {
			for range ticker.C {
				log.Printf("Brightness: %d, Temp: %dºC, Humidity: %d%%", sData.brightness, sData.temperature, sData.humidity)
			}
		}(sData)

		// Add task functions
		go devicesController.ControlDesktopBrightness(sData.brightness)
		go devicesController.ControlKbdBacklightLaptop(sData.brightness)
	}
	defer wg.Done()
}
