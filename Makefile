.PHONY: build deploy build-client run listen deploy upgrade bots validator

help: 								## Show help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n"} /^[$$()% 0-9a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-21s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

solana-test-validator:				## Start a local validator with all program dependencies
	solana-test-validator

build:								## Build all programs
	./scripts/build.sh

unit-test:							## Build and run unit tests. Usage: make unit-test test=tests_liquidation_calcs
	$(eval _ARGS=${ARGS})
ifdef test
	$(eval _ARGS+= --tests ${test} -- --nocapture)
endif
	RUST_MIN_STACK=83886080 cargo test --features localnet --lib ${_ARGS}

stress-test:							## Build and run stress tests
	RUST_MIN_STACK=83886080 cargo test --lib --features localnet stress_test

bpf-test:							## Build and run BPF integration tests. Usage: make bpf-test RUST_LOG=fatal RUST_BACKTRACE=full test=tests_borrowing_operations RUST_ARGS="--test-threads=1"
	$(eval _ARGS=${ARGS})
ifdef RUST_LOG
	$(eval RUST_LOG=${RUST_LOG})
else
	$(eval RUST_LOG=error,solana_runtime::message_processor::stable_log=debug)
endif
ifdef RUST_BACKTRACE
	$(eval RUST_BACKTRACE=${RUST_BACKTRACE})
else
	$(eval RUST_BACKTRACE=1)
endif
ifdef test
	$(eval _ARGS+= --test ${test})
endif
ifdef RUST_ARGS
	$(eval _ARGS+= -- ${RUST_ARGS})
else 
	$(eval _ARGS+= -- --nocapture)
endif
	RUST_MIN_STACK=83886080 RUST_LOG=${RUST_LOG} RUST_BACKTRACE=${RUST_BACKTRACE} cargo test-bpf --features localnet integration_tests edition2021  ${_ARGS}