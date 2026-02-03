set -x

aws cognito-idp admin-initiate-auth --user-pool-id ${AWS_COGNITO_USER_POOL_ID} --client-id ${AWS_COGNITO_CLIENT_ID} --auth-flow ADMIN_USER_PASSWORD_AUTH --auth-parameters USERNAME="${AWS_COGNITO_USERNAME}",PASSWORD="${AWS_COGNITO_PASSWORD}"
