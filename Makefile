run:
	LIBGL_ALWAYS_SOFTWARE=1 cargo run puzzles/puzzle.sdm

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture
