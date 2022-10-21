package devicesController

import (
	"example.com/utils"
)

func RpiTurnOnSocket1() string {
	ret := utils.Execute("ssh", "pi@192.168.1.107", "python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on")
	return ret
}

func RpiTurnOffSocket1() string {
	ret := utils.Execute("ssh", "pi@192.168.1.107", "python3 ~/rush/energienie.py 1=off && python3 ~/rush/energienie.py 1=off && python3 ~/rush/energienie.py 1=off")
	return ret
}
