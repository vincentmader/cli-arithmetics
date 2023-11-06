run:
	mkdir -p ./saves
	cargo run --release
setup:
	cd bin && ./setup.sh
stats:
	cargo run --bin show_stats
	make plots
plots:
	cd src/bin/ && ./show_plots.py
