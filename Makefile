# INFO: Project variables
.PHONY: run start build commit add test getcover cover clean

default: run

# INFO: Project commands
run:
	@go run src/main.go $(cmd)

start:
	@./build/louarch $(cmd)

startclean: build
	@./build/louarch $(cmd)

build:
	@go build -o ./build/louarch src/main.go

clean:
	@rm -rf ./build/

help:
	@echo "run       run the app"
	@echo "start     build and run the app"
	@echo "build     build the app"
	@echo "clean     clean the build directory"
