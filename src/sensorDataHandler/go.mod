module example.com/sensorDataHandler

go 1.19

replace example.com/devicesController => ../devicesController

require (
	example.com/devicesController v0.0.0-00010101000000-000000000000
	github.com/joho/godotenv v1.4.0
)
