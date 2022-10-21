module example.com/sensorDataHandler

go 1.19

replace example.com/desktopController => ../desktopController

require (
	example.com/desktopController v0.0.0-00010101000000-000000000000
	github.com/joho/godotenv v1.4.0
)
