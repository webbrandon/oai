.PHONY: build

build:
	cargo build --release

install:
	@sudo chmod +x target/release/oai
	@sudo cp target/release/oai /usr/local/bin/
	@echo "Install Complete"

examples: conversation-summary-example javascript-code-review-example rust-code-improvement-example create-html-component-example edit-sentence-example edit-code-example create-image-example edit-image-example ask-intention-example

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

edit-sentence-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Prompt: Door swing open and shut when the wind blows down the halls."
	@echo "Instructions: Correct the sentence with proper english grammer."
	@echo "Response ----------------------------------------------------------------------"
	@oai -m text-davinci-edit-001 -t 1.2 "Door swing open and shut when the wind blows down the halls." "Correct the sentence with proper english grammer."
	@echo "\n\n"

edit-code-example:
	@echo "Example -----------------------------------------------------------------------"
	@cat examples/refactor
	@echo "Instructions: Refactor for reuse in rust."
	@echo "Response ----------------------------------------------------------------------"
	@oai -m code-davinci-edit-001 --max-tokens 236 @examples/refactor "Refactor for reuse in rust."
	@echo "\n\n"

create-image-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Hand drawn rose on mountain top with the sun beaming down on it done with pencil only."
	@echo "Response ----------------------------------------------------------------------"
	@oai image --save ~/.openai "Hand drawn rose on mountain top with the sun beaming down on it done with pencil only."

edit-image-example:
	@echo "Example -----------------------------------------------------------------------"
	@echo "Create a galatic adventure in the background."
	@echo "Response ----------------------------------------------------------------------"
	@oai image --save ~/.openai -i examples/output.png -m examples/output_mask.png "Create a galatic adventure in the background."
