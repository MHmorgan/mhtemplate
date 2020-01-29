
all :

test :
	cargo test -- --nocapture

test-release :
	cargo test --release -- --nocapture

doc :
	cargo doc --offline --no-deps --open