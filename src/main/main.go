package main

import (
	"example.com/devicesController"
	"example.com/sensorDataHandler"
	"example.com/telegramBot"
	"example.com/utils"
	"sync"
	// "fmt"
)

func main() {
	utils.ParseEnvVars()

	var wg = &sync.WaitGroup{}
	var sData sensorDataHandler.SensorData
	wg.Add(6)

	go telegramBot.PollUpdates(wg)

	go devicesController.UpdateDesktop1Status(wg, sData.Brightness, 1)
	go devicesController.UpdateDesktop2Status(wg, 1)
	go devicesController.UpdateDesktop3Status(wg, sData.Brightness, 1)

	go sensorDataHandler.ReadSensorData(wg, &sData)

	go sensorDataHandler.HandleSensorData(wg, &sData)

	wg.Wait()
}
