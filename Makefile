
all :

test :
	cargo test -- --nocapture

test-release :
	cargo test --release -- --nocapture

doc :
	cargo doc --offline --no-deps --open

# Prepare for release
prepare :
	cargo fix --workspace --edition-idioms --release
	cargo clippy --verbose
	cargo fmt
