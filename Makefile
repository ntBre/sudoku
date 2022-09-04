run:
	LIBGL_ALWAYS_SOFTWARE=1 cargo run

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture
