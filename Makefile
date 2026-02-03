build:
	ulimit -n 4096 && cargo lambda build --arm64 --release

distclean:
	rm -r target
	rm -r src/target

deploy: build
	cargo lambda deploy --env-var AWS_COGNITO_CLIENT_ID=${AWS_COGNITO_CLIENT_ID},AWS_COGNITO_USER_POOL_ID=${AWS_COGNITO_USER_POOL_ID},AWS_COGNITO_REGION=${AWS_COGNITO_REGION}

