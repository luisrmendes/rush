package devicesController

import (
	"rush/utils"
)

func RpiTurnOnSockets() string {
	ret := utils.Execute("ssh", "pi@192.168.1.107",
		"python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on && "+
			"python3 ~/rush/energienie.py 2=on && python3 ~/rush/energienie.py 2=on && python3 ~/rush/energienie.py 2=on")
	return ret
}

func RpiTurnOffSockets() string {
	ret := utils.Execute("ssh", "pi@192.168.1.107",
		"python3 ~/rush/energienie.py 1=off && python3 ~/rush/energienie.py 1=off && python3 ~/rush/energienie.py 1=off && "+
			"python3 ~/rush/energienie.py 2=off && python3 ~/rush/energienie.py 2=off && python3 ~/rush/energienie.py 2=off")
	return ret
}
