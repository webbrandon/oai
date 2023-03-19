.PHONY: build

build:
	cargo build --release

install:
	@sudo chmod +x target/release/oai
	@sudo cp target/release/oai /usr/local/bin/
	@echo "Install Complete"

examples: conversation-summary-example javascript-code-review-example rust-code-improvement-example create-html-component-example ask-intention-example

javascript-code-review-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Please review the code and tell me if there are mistakes:"
	@cat examples/index.js
	@echo "Response ----------------------------------------------------------------------"
	@{ echo "Please review the code and tell me if there are mistakes:"; cat examples/index.js; } | oai
	@echo "\n\n"

rust-code-improvement-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Please give a coded modification with a generic in the following Rust module:"
	@cat examples/test
	@echo "Response ----------------------------------------------------------------------"
	@{ echo "Please give a coded modification with a generic in the following Rust module:"; cat examples/test; } | oai
	@echo "\n\n"

conversation-summary-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Please give me a summary of the following conversation:"
	@cat examples/convo.txt
	@echo "Response ----------------------------------------------------------------------"
	@{ echo "Please give me a summary of the following conversation:"; cat examples/convo.txt; } | oai
	@echo "\n\n"

create-html-component-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Write an HTML component with shadow DOM that ingest a style object and data object to create a button that displays a modal with data and allows the user to escape modal with the keyboards to escape key or exit button."
	@echo "Response ----------------------------------------------------------------------"
	@oai -t 1.2 "Write an HTML component with shadow DOM that ingest a style object and data object to create a button that displays a modal with data and allows the user to escape modal with the keyboards to escape key or exit button." > examples/button.html
	@cat examples/button.html
	@echo "\n\n"

ask-intention-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Do you plan on becoming our overlord and supreme ruler?"
	@echo "Response ----------------------------------------------------------------------"
	@oai "Do you plan on becoming our overlord and supreme ruler?"
	@echo "\n\n"
