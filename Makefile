run:
	mkdir -p ./saves
	cargo run --release
stats:
	cargo run --bin show_stats
	make plots
plots:
	./src/bin/show_plots.py
