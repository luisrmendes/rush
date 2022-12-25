module example.com/devicesController

go 1.18

replace example.com/utils => ../utils

require (
	example.com/utils v0.0.0-00010101000000-000000000000
	github.com/go-ping/ping v1.1.0
)

require (
	github.com/google/uuid v1.3.0 // indirect
	github.com/joho/godotenv v1.4.0 // indirect
	golang.org/x/net v0.4.0 // indirect
	golang.org/x/sync v0.1.0 // indirect
	golang.org/x/sys v0.3.0 // indirect
)
