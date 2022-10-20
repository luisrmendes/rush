package sensorDataHandler

import (
	"fmt"
	"github.com/joho/godotenv"
	"net"
	"os"
	"sync"
)


func ReadSensorData(wg *sync.WaitGroup) {
	err := godotenv.Load()
	if err != nil {
		fmt.Println("Some error occured. Err: %s", err)
	}

	esp8266_address_port := os.Getenv("ESP8266_ADDRESS_PORT")

	connection, err := net.Dial("tcp", esp8266_address_port)
	if err != nil {
		panic(err)
	}
	for {
		buffer := make([]byte, 1024)
		mLen, err := connection.Read(buffer)
		if err != nil {
			fmt.Println("Error reading:", err.Error())
		}
		fmt.Println("Received: ", string(buffer[:mLen]))
	}
	defer wg.Done()
}
