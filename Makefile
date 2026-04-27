.PHONY: run

run:
	dx serve --release

bundle-web:
	dx bundle --web --release
