use lambda_runtime::{handler_fn, Context, Error};
use slack_bot_lambda::{run, Output};

async fn func(_: std::collections::HashMap<String, String>, _: Context) -> Result<Output, Error> {
    Ok(run().await.unwrap_or(Output {
        message: "error".to_string(),
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    openssl_probe::init_ssl_cert_env_vars();
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}
