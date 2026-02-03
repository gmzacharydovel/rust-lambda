build:
	ulimit -n 4096 && cargo lambda build --arm64

deploy:
	cargo lambda deploy

clean:
	rm -r target
	rm -r src/target
