FROM public.ecr.aws/lambda/provided:al2

COPY ./target/x86_64-unknown-linux-musl/release/slack-bot-lambda ${LAMBDA_RUNTIME_DIR}/bootstrap
COPY ./.env ${LAMBDA_TASK_ROOT}/.env

CMD [ "lambda-handler" ]