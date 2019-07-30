CURRENT_DIRECTORY := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

all:
	@echo "Please choose a task."
.PHONY: all

build.github:
	docker run --rm -it -v $(CURRENT_DIRECTORY):/home/rust/src ekidd/rust-musl-builder cargo build --release --features "lambda-github" --bin lambda_github
	mv ./target/x86_64-unknown-linux-musl/release/lambda_github ./target/x86_64-unknown-linux-musl/release/bootstrap
	zip -j lambda_github.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
.PHONY: build

deploy.github:
	aws lambda update-function-code --function-name lendabot-github --zip-file fileb://lambda_github.zip
.PHONY: deploy

build.slack:
	docker run --rm -it -v $(CURRENT_DIRECTORY):/home/rust/src ekidd/rust-musl-builder cargo build --release --features "lambda-slack" --bin lambda_slack
	mv ./target/x86_64-unknown-linux-musl/release/lambda_slack ./target/x86_64-unknown-linux-musl/release/bootstrap
	zip -j lambda_slack.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
.PHONY: build

deploy.slack:
	aws lambda update-function-code --function-name lendabot-slack --zip-file fileb://lambda_slack.zip
.PHONY: deploy
