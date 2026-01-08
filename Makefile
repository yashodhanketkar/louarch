# INFO: Project variables
.PHONY: run start build commit add test getcover cover clean

default: run

# INFO: Project commands
run:
	@go run . $(cmd)

start:
	@./build/louarch $(cmd)

startclean: build
	@./build/louarch $(cmd)

build: requirements
	@go build -o ./build/louarch .

clean:
	@rm -rf ./build/

requirements:
	@chmod +x ./check-requirements.sh
	@./check-requirements.sh

install: build
	@chmod +x ./install.sh
	@./install.sh

uninstall:
	@chmod +x ./uninstall.sh
	@./uninstall.sh

help:
	@echo "run       run the app"
	@echo "start     build and run the app"
	@echo "build     build the app"
	@echo "clean     clean the build directory"
