package main

import (
	"example.com/sensorDataHandler"
	"example.com/telegramBot"
	"sync"
)

func main() {
	var wg = &sync.WaitGroup{}

	wg.Add(1)
	go telegramBot.PollUpdates(wg)
	wg.Add(1)
	go sensorDataHandler.ReadSensorData(wg)

	wg.Wait()
}
