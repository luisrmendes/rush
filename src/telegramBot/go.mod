module rush/telegramBot

go 1.18

require (
	rush/devicesController v0.0.0-00010101000000-000000000000
	rush/utils v0.0.0-00010101000000-000000000000
	github.com/go-telegram-bot-api/telegram-bot-api/v5 v5.5.1
	github.com/joho/godotenv v1.4.0
)

replace rush/main => ../main

replace rush/utils => ../utils

replace rush/devicesController => ../devicesController

replace rush/devicesController => ../devicesController
