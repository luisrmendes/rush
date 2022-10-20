package sensorDataHandler

import (
	"fmt"
	"github.com/joho/godotenv"
	"log"
	"net"
	"os"
	"sync"
)

func ReadSensorData(wg *sync.WaitGroup) {
	esp8266_address_port := os.Getenv("ESP8266_ADDRESS_PORT")
	if esp8266_address_port == "" {
		log.Printf("ESP8266_ADDRESS_PORT not found in env vars, checking .env")
		err := godotenv.Load(".env")
		if err != nil {
			log.Fatalf("Some error occured. Err: %s", err)
		}
	}
	esp8266_address_port = os.Getenv("ESP8266_ADDRESS_PORT")
	err := godotenv.Load()
	if err != nil {
		fmt.Println("Some error occured. Err: %s", err)
	}

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
