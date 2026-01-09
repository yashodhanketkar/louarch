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
	@chmod +x ./scripts/check-requirements.sh
	@./scripts/check-requirements.sh

install: build
	@chmod +x ./scripts/install.sh
	@./scripts/install.sh

uninstall:
	@chmod +x ./scripts/uninstall.sh
	@./scripts/uninstall.sh

help:
	@echo "run       run the app"
	@echo "start     build and run the app"
	@echo "build     build the app"
	@echo "clean     clean the build directory"
