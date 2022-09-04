run:
	LIBGL_ALWAYS_SOFTWARE=1 cargo run puzzles/easy.sdm

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture
