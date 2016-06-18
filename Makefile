build:
	cargo build --release
	
run:
	cat input/large.in | target/release/mutual_friendly 4
	
scalingtest:
	sh ./scalingtest.sh
