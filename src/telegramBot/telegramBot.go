package telegramBot

import (
	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	"log"
	"os"
	"rush/devicesController"
	"rush/utils"
	"sync"
)

type Command struct {
	name string
}

// Implements functionality of each telegram command
func (c Command) Handler() string {
	switch c.name {

	case "get_system_status":
		return devicesController.GetSystemStatus()

	case "enable_brightness_auto_control":
		return devicesController.EnableAutomaticBrightnessControl()

	case "disable_brightness_auto_control":
		return devicesController.DisableAutomaticBrightnessControl()

	case "ipv4":
		return utils.Execute("dig", "@resolver1.opendns.com", "A",
			"myip.opendns.com", "+short", "-4")

	case "desktop_wakeup":
		return utils.Execute("wol", "00:D8:61:a1:CE:00")

	case "lights_on":
		return devicesController.RpiTurnOnSockets()

	case "lights_off":
		return devicesController.RpiTurnOffSockets()

	default:
		log.Printf("Command %s handler not implemented!", c.name)
		return "Command " + c.name + " handler not implemented!"
	}
}

func HandleTelegramCommands(receivedMessage *tgbotapi.Message) string {
	log.Printf("Received a command: %s", receivedMessage.Text)
	command := Command{receivedMessage.Text[1:]}
	return command.Handler()
}

// Polling daemon
// Receives telegram bot updates, calls HandleTelegramCommands
func PollUpdates(wg *sync.WaitGroup) {
	bot, err := tgbotapi.NewBotAPI(os.Getenv("TELEGRAM_API_KEY"))
	if err != nil {
		log.Println(err)
	}

	log.Printf("Authorized on account %s", bot.Self.UserName)
	u := tgbotapi.NewUpdate(0)
	u.Timeout = 10
	updates := bot.GetUpdatesChan(u)

	// Welcome message
	msg := tgbotapi.NewMessage(322011297, "I'm online!")
	bot.Send(msg)

	for update := range updates {
		if update.Message != nil { // If we got a message
			if len(update.Message.Text) > 0 && update.Message.Text[0:1] == "/" {
				msg := tgbotapi.NewMessage(update.Message.Chat.ID, HandleTelegramCommands(update.Message))
				// msg.ReplyToMessageID = update.Message.MessageID
				bot.Send(msg)
			} else {
				// msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
				// msg.ReplyToMessageID = update.Message.MessageID
				// bot.Send(msg)
			}
		}
	}
}
