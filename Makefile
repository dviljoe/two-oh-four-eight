.PHONY: run clean

run:
	cargo run --features bevy/dynamic_linking

clean:
	rm -rf target/
