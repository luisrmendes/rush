package main

import (
	"example.com/telegramBot"
	"example.com/sensorDataHandler"
	"sync"

)

func main() {
	var wg = &sync.WaitGroup{}
	var sData sensorDataHandler.SensorData
	wg.Add(3)

	go telegramBot.PollUpdates(wg)

	go sensorDataHandler.ReadSensorData(wg, &sData)

	go sensorDataHandler.HandleSensorData(wg, &sData)

	wg.Wait()
}
