package telegramBot

import (
	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	"github.com/joho/godotenv"
	"example.com/utils"
	"example.com/devicesController"
	"log"
	"os"
	"sync"
)

type Command struct {
	name string
}

// Implements each telegram command
func (c Command) Handler() string {
	switch c.name {

	case "ipv4":
		return utils.Execute("dig", "@resolver1.opendns.com", "A",
			"myip.opendns.com", "+short", "-4")

	case "desktop_wakeup":
		return utils.Execute("wakeonlan", "00:D8:61:a1:CE:00")

	case "lights_on":
		return devicesController.RpiTurnOnSocket1()

	case "lights_off":
		return devicesController.RpiTurnOffSocket1()
		
	default:
		log.Printf("Command %s handler not implemented!", c.name)
		return "Command " + c.name + " handler not implemented!"
	}
}

func HandleCommands(receivedMessage *tgbotapi.Message) string {
	log.Printf("Received a command: %s", receivedMessage.Text)
	command := Command{receivedMessage.Text[1:]}
	return command.Handler()
}

// Polls updates from the bot API
// Calls HandleUpdates to handle ... updates
func PollUpdates(wg *sync.WaitGroup) {
	telegram_api_key := os.Getenv("TELEGRAM_API_KEY")
	if telegram_api_key == "" {
		log.Printf("TELEGRAM_API_KEY not found in env vars, checking .env")
		err := godotenv.Load(".env")
		if err != nil {
			log.Println("Some error occured. Err: %s", err)
		}
	}
	telegram_api_key = os.Getenv("TELEGRAM_API_KEY")

	bot, err := tgbotapi.NewBotAPI(telegram_api_key)
	if err != nil {
		log.Println(err)
	}

	// bot.Debug = true

	log.Printf("Authorized on account %s", bot.Self.UserName)

	u := tgbotapi.NewUpdate(0)
	u.Timeout = 10

	updates := bot.GetUpdatesChan(u)

	// Welcome message
	msg := tgbotapi.NewMessage(322011297, "I'm online!")
	bot.Send(msg)

	for update := range updates {
		if update.Message != nil { // If we got a message
			if update.Message.Text[0:1] == "/" {
				msg := tgbotapi.NewMessage(update.Message.Chat.ID, HandleCommands(update.Message))
				// msg.ReplyToMessageID = update.Message.MessageID
				bot.Send(msg)
			} else {
				msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
				// msg.ReplyToMessageID = update.Message.MessageID
				bot.Send(msg)
			}
		}
	}
	defer wg.Done()
}
