mod domain;
mod eventbridge_port;

use crate::eventbridge_port::cart_clear_user_deleted_event_port;
use eventing::{EventingPort, events::user::user_deleted::EventUserDeletedV1};

use lambda_adaptor::common_lambda_adaptor;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use aws_lambda_events::cloudwatch_events::CloudWatchEvent;

use models::models::cart::CartRepositoryPort;

async fn eventbridge_lambda_driving_adaptor<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    event: LambdaEvent<CloudWatchEvent<EventUserDeletedV1>>,
) -> Result<(), Error> {
    cart_clear_user_deleted_event_port(cart_repository_port, eventing_port, event.payload)
        .await
        .unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Common snippit from all lambda functions
    common_lambda_adaptor!();
    {
        // Provision required repositories once in the main function
        let sdk_credential_meta_repository =
            sdk_credential_meta_repository::SdkCredentialsMetaRepository::new().await;
        let dynamo_db_repository = persistance_repository::DynamoDBSingleTableRepository::new(
            &sdk_credential_meta_repository,
        );
        let eventing_repository =
            eventing::EventingRepository::new(&sdk_credential_meta_repository);
        let cart_repository =
            models::models::cart::CartRepositoryAdaptor::new(&dynamo_db_repository);

        run(service_fn(|event| {
            eventbridge_lambda_driving_adaptor(&cart_repository, &eventing_repository, event)
        }))
        .await
    }
}
