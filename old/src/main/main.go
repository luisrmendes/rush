package main

import (
	"rush/devicesController"
	"rush/esp8266Controller"
	"rush/telegramBot"
	"rush/utils"
	"sync"
	// "fmt"
)

func main() {
	utils.ParseEnvVars()

	var wg = &sync.WaitGroup{}
	var sData esp8266Controller.SensorData
	wg.Add(6)

	go telegramBot.PollUpdates(wg)

	go devicesController.UpdateSystem1Status(wg, sData.Brightness, 1)
	go devicesController.UpdateSystem2Status(wg, 1)
	// go devicesController.UpdateDesktop3Status(wg, sData.Brightness, 1)

	go esp8266Controller.ReadSensorData(wg, &sData)

	go esp8266Controller.HandleSensorData(wg, &sData)

	wg.Wait()
}
