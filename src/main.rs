use whirly_rs::configuration::get_conf;
use whirly_rs::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_conf().expect("failed to properly load configuration");
    let application = Application::build(configuration).await?;

    application.run_until_stop().await?;
    Ok(())
}
