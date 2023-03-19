# TODO
- [ ] Test the methods
- [ ] Refector code and extract some functionality into re-useable functions
- [ ] Improve robustness with unit & integration tests
- [ ] Write comments and explanations
- [ ] Validate parsed input parameters received from the cmdln parser
- [ ] Implement helper for setting user env variable for logging
- [ ] Add automated travis CI/CD with rustfmt & build tests

# BACKLOG
- [ ] Packaging of gbt chatbot application into shared rpm/deb with Linux service capabilities
- [ ] Wrap the telnet server footprint into a future compatible crate for easier testing capabilities
- [ ] Implement bg support for tokenization/prediction non-blocking batch process execution
- [ ] Introduce Return Value based handling mechanism of json RPC responses
- [ ] Add more Comments to improve readability
- [ ] Add Documentation of functions

# DONE
- [x] Create structure skeleton
- [x] initialize Logging with env_logger
- [x] use tokio: :main to run the future in try!()
- [x] use StructOpt for parsing command line arguments

Generated using 'oai' given examples/todo-md.md as premise to write code improvement TODO list.
