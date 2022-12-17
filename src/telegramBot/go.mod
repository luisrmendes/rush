module example.com/telegramBot

go 1.18

require (
	example.com/devicesController v0.0.0-00010101000000-000000000000
	example.com/utils v0.0.0-00010101000000-000000000000
	github.com/go-telegram-bot-api/telegram-bot-api/v5 v5.5.1
	github.com/joho/godotenv v1.4.0
)

replace example.com/main => ../main

replace example.com/utils => ../utils

replace example.com/devicescontroller => ../devicesController

replace example.com/devicesController => ../devicesController
