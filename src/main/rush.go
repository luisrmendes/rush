package main

import (
	// "example.com/telegramBot"
	"example.com/utils"
	"github.com/joho/godotenv"
	"log"
	"net"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"
)

type sensorData struct {
	brightness  int
	temperature int
	humidity    int
}

// Looping function
// Reads sensor data from ESP8266 TCP server
// Handles if TCP server shutdowns
// Calls handleSensorData
func readSensorData(wg *sync.WaitGroup, sData *sensorData) {
	// Fetch environment variables
	esp8266_address_port := os.Getenv("ESP8266_ADDRESS_PORT")
	if esp8266_address_port == "" {
		log.Printf("ESP8266_ADDRESS_PORT not found in env vars, checking .env")
		err := godotenv.Load(".env")
		if err != nil {
			log.Println("Some error occured. Err: %s", err)
		}
	}
	esp8266_address_port = os.Getenv("ESP8266_ADDRESS_PORT")
	err := godotenv.Load()
	if err != nil {
		log.Println("Some error occured. Err: %s", err)
	}

	for {
		connection, err := net.Dial("tcp", esp8266_address_port)
		if err != nil {
			panic(err)
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

func handleSensorData(wg *sync.WaitGroup, sData *sensorData) {
	for {
		time.Sleep(2000000000)
		if sData.brightness == 0 {
			continue
		}
		log.Printf("Brightness: %d, Temp: %dºC, Humidity: %d%%", sData.brightness, sData.temperature, sData.humidity)
		utils.ControlDesktopBrightness(sData.brightness)
	}
	defer wg.Done()
}

func main() {
	var wg = &sync.WaitGroup{}
	var sData sensorData

	wg.Add(3)
	// go telegramBot.PollUpdates(wg)

	go readSensorData(wg, &sData)

	go handleSensorData(wg, &sData)

	wg.Wait()
}
