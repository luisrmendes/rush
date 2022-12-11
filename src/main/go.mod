module example.com/main

go 1.18

require (
	example.com/sensorDataHandler v0.0.0-00010101000000-000000000000
	example.com/telegramBot v0.0.0-00010101000000-000000000000
)

require (
	example.com/devicesController v0.0.0-00010101000000-000000000000 // indirect
	example.com/utils v0.0.0-00010101000000-000000000000 // indirect
	github.com/go-telegram-bot-api/telegram-bot-api/v5 v5.5.1 // indirect
	github.com/joho/godotenv v1.4.0 // indirect
)

replace example.com/telegramBot => ../telegramBot

replace example.com/devicesController => ../devicesController

replace example.com/sensorDataHandler => ../sensorDataHandler

replace example.com/utils => ../utils
