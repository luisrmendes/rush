package main

import (
	// "example.com/sensorDataHandler"
	// "example.com/telegramBot"
	// "sync"
	"example.com/utils"
	// "fmt"
)

func main() {

	var pq utils.PriorityQueue
	var newElem utils.PqElement
	newElem.Priority = 1
	newElem.Name = "temples of syrinx"

	utils.InsertElement(&pq, newElem)

	var newElem2 utils.PqElement
	newElem2.Priority = 2
	newElem2.Name = "snowdog"

	utils.InsertElement(&pq, newElem2)

	var newElem3 utils.PqElement
	newElem3.Priority = 3
	newElem3.Name = "2112"

	utils.InsertElement(&pq, newElem3)

	var newElem4 utils.PqElement
	newElem4.Priority = 2
	newElem4.Name = "by tor"

	utils.InsertElement(&pq, newElem4)

	utils.Print(pq)


	// var wg = &sync.WaitGroup{}
	// var sData sensorDataHandler.SensorData
	// wg.Add(3)

	// go telegramBot.PollUpdates(wg)

	// go sensorDataHandler.ReadSensorData(wg, &sData)

	// go sensorDataHandler.HandleSensorData(wg, &sData)

	// wg.Wait()
}
