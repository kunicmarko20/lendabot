CURRENT_DIRECTORY := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

all:
	@echo "Please choose a task."

build:
	docker run --rm -it -v "$(CURRENT_DIRECTORY)/../lendabot":/home/rust/lendabot -v $(CURRENT_DIRECTORY):/home/rust/src ekidd/rust-musl-builder cargo build --release
	zip -j lambda.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

deploy:
	aws lambda update-function-code --function-name lendabot-github --zip-file fileb://lambda.zip
