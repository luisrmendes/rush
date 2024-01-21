module rush/esp8266Controller

go 1.18

replace rush/devicesController => ../devicesController

require (
	rush/devicesController v0.0.0-00010101000000-000000000000
	github.com/joho/godotenv v1.4.0
)
