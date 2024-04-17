use std::collections::HashMap;

use super::common::get_or_init_config;

use orchestrator::config::Config;
use orchestrator::jobs::types::{
        JobItem,
        JobType::DataSubmission,
        JobStatus::Created,
        ExternalId,
};
use rstest::*;
use starknet::providers::Provider;

use starknet::core::types::FieldElement;
use ::uuid::Uuid;

#[fixture]
fn vec_of_field_elements() -> Vec<FieldElement> {
    vec![FieldElement::ONE, FieldElement::TWO, FieldElement::THREE]
}

#[rstest(get_or_init_config(String::from("http://localhost:9944")))]
#[tokio::test]
async fn test_valid_config(
    #[future] get_or_init_config: &Config,
) {
    let config = get_or_init_config.await;
    config.starknet_client();
    config.da_client();
    config.database();
    config.queue();
}

#[ignore = "Run this separately as it will fail the other tests (config can't be created with each test)"]
#[rstest(get_or_init_config(String::from("http://invalid:invalid")))]
#[should_panic]
#[tokio::test]
async fn test_invalid_config(
    #[future] get_or_init_config: &Config,
) {
    get_or_init_config.await;
}

#[rstest]
#[tokio::test]
async fn test_config_starknet_client(
    #[future] get_or_init_config: &Config,
) {
    let config = get_or_init_config.await;
    let result = config.starknet_client().block_number().await;
    assert!(result.is_ok(), "Failed to run starknet_client()");
}

#[rstest]
#[should_panic]
#[tokio::test]
async fn test_config_da_client(
    #[future] get_or_init_config: &Config,
    vec_of_field_elements: Vec<FieldElement>,
) {
    let config = get_or_init_config.await;
    
    let result = config.da_client().publish_state_diff(vec_of_field_elements).await;
    assert!(result.is_err());
}

#[rstest]
#[should_panic]
#[tokio::test]
async fn test_config_database(
    #[future] get_or_init_config: &Config,
) {
    let config = get_or_init_config.await;
    let job = JobItem { 
        id: Uuid::new_v4(),
        internal_id: String::from("0"),
        job_type: DataSubmission,status: Created,
        external_id: ExternalId::Number(0),
        metadata: HashMap::new(),
        version: 0,
    };
    let result = config.database().create_job(job).await;
    assert!(result.is_err());
}