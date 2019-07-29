CURRENT_DIRECTORY := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

all:
	@echo "Please choose a task."
.PHONY: all

build.github:
	docker run --rm -it -v $(CURRENT_DIRECTORY):/home/rust/src ekidd/rust-musl-builder cargo build --release --features "lambda-github"
	mv ./target/x86_64-unknown-linux-musl/release/lambda_github ./target/x86_64-unknown-linux-musl/release/bootstrap
	zip -j lambda_github.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
.PHONY: build

deploy.github:
	aws lambda update-function-code --function-name lendabot --zip-file fileb://lambda_github.zip
.PHONY: deploy
