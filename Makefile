.PHONY: check-format
check-format:
	cargo fmt --check

.PHONY: check-tests
check-tests:
	cargo test --all-features

.PHONY: check-lint
check-lint:
	cargo clippy

.PHONY: check
check: | check-lint check-tests check-format

.PHONY: format
format:
	cargo fmt
