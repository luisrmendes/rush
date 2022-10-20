module example.com/main

go 1.19

replace example.com/greetings => ../greetings

replace example.com/telegramBot => ../telegramBot

require (
	example.com/utils v0.0.0-00010101000000-000000000000
	github.com/joho/godotenv v1.4.0
)

require (
	example.com/telegramBot v0.0.0-00010101000000-000000000000 // indirect
	github.com/go-telegram-bot-api/telegram-bot-api/v5 v5.5.1 // indirect
)

replace example.com/utils => ../utils
