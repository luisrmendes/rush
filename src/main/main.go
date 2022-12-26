package main

import (
	"example.com/devicesController"
	"example.com/sensorDataHandler"
	"example.com/telegramBot"
	"example.com/utils"
	"os"
	"sync"
	// "fmt"
)

func main() {
	utils.ParseEnvVars()

	var wg = &sync.WaitGroup{}
	var sData sensorDataHandler.SensorData
	wg.Add(4)

	go telegramBot.PollUpdates(wg)

	go devicesController.UpdateSystemStatus(wg, sData.brightness, os.Getenv("SYSTEM_1_ADDRESS"), 2)

	go sensorDataHandler.ReadSensorData(wg, &sData)

	go sensorDataHandler.HandleSensorData(wg, &sData)

	wg.Wait()
}
