module example.com/main

go 1.19

replace example.com/greetings => ../greetings

replace example.com/telegramBot => ../telegramBot

require (
	example.com/utils v0.0.0-00010101000000-000000000000
	github.com/joho/godotenv v1.4.0
)

replace example.com/utils => ../utils
