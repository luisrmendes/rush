module example.com/main

go 1.18

require (
	example.com/devicesController v0.0.0-00010101000000-000000000000
	example.com/sensorDataHandler v0.0.0-00010101000000-000000000000
	example.com/telegramBot v0.0.0-00010101000000-000000000000
	example.com/utils v0.0.0-00010101000000-000000000000
)

require (
	github.com/go-ping/ping v1.1.0 // indirect
	github.com/go-telegram-bot-api/telegram-bot-api/v5 v5.5.1 // indirect
	github.com/google/uuid v1.3.0 // indirect
	github.com/joho/godotenv v1.4.0 // indirect
	golang.org/x/net v0.4.0 // indirect
	golang.org/x/sync v0.1.0 // indirect
	golang.org/x/sys v0.3.0 // indirect
)

replace example.com/telegramBot => ../telegramBot

replace example.com/sensorDataHandler => ../sensorDataHandler

replace example.com/utils => ../utils

replace example.com/devicesController => ../devicesController
