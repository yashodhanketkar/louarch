# INFO: Project variables
.PHONY: run start build commit add test getcover cover clean

default: run

# INFO: Project commands
run:
	@go run main/main.go

start:
	@./build/wallswitcher

startclean: build
	@./build/wallswitcher

build:
	@go build -o ./build/wallswitcher main/main.go

clean:
	@rm -rf ./build/

help:
	@echo "run       run the app"
	@echo "start     build and run the app"
	@echo "build     build the app"
	@echo "clean     clean the build directory"
