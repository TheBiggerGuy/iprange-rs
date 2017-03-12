build:
	clear; cargo build
test:
	clear; RUST_TEST_TASKS=1 RUST_BACKTRACE=0 RUST_LOG=iprange=debug cargo test --no-fail-fast -- --nocapture
