use aws_config;
use aws_config::BehaviorVersion;
use aws_sdk_sts::config::ProvideCredentials;
use colored::Colorize;
use deltalake::arrow::record_batch::RecordBatch;
use deltalake::datafusion::execution::context::SessionContext;
use deltalake::{open_table_with_storage_options, DeltaOps};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

// Load AWS Creds into a hashmap for use with delta lake reader
pub async fn get_aws_config() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // Locate aws creds
    let config = aws_config::defaults(BehaviorVersion::latest())
        .retry_config(aws_config::retry::RetryConfig::standard().with_max_attempts(5))
        .timeout_config(
            aws_config::timeout::TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(30))
                .build(),
        )
        .load()
        .await;

    // Create new hashmap to store creds
    let mut aws_info = HashMap::new();

    // Add credentials to HashMap if available
    if let Some(creds_provider) = config.credentials_provider() {
        match creds_provider.provide_credentials().await {
            Ok(creds) => {
                aws_info.insert(
                    "AWS_ACCESS_KEY_ID".to_string(),
                    creds.access_key_id().to_string(),
                );
                aws_info.insert(
                    "AWS_SECRET_ACCESS_KEY".to_string(),
                    creds.secret_access_key().to_string(),
                );
                if let Some(session_token) = creds.session_token() {
                    aws_info.insert("AWS_SESSION_TOKEN".to_string(), session_token.to_string());
                }
            }
            Err(e) => return Err(format!("Failed to retrieve credentials: {}", e).into()),
        }
    } else {
        return Err("No credentials provider found in the configuration".into());
    }

    Ok(aws_info)
}

// Read DeltaLake table stored in S3
pub async fn load_remote_delta_lake_table_info(
    s3_uri: &str,
    credential_hash_map: HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // load credentials
    let storage_options: HashMap<String, String> = credential_hash_map;

    // register aws backend
    deltalake_aws::register_handlers(None);

    // open delta lake table
    let table = match open_table_with_storage_options(s3_uri, storage_options).await {
        Ok(tbl) => tbl,
        Err(_) => {
            let ops = DeltaOps::try_from_uri(s3_uri).await?;
            ops.create().with_table_name("data").await?
        }
    };

    // Start DataFusion context
    let ctx = SessionContext::new();

    // Register table
    ctx.register_table("data", Arc::new(table))
        .map_err(|e| format!("Failed to register table: {}", e))?;

    // Create batches
    let batches = ctx
        .sql("SELECT * FROM data")
        .await
        .map_err(|e| format!("SQL Query Failed: {}", e))?
        .collect()
        .await
        .map_err(|e| format!("Could not read results: {}", e))?;

    for batch in batches {
        println!("{}", "DeltaLake Output: Columns and RecordBatch.".green());
        process_record_batch(&batch);
        println!("Record Batch: {:?}", batch);
    }

    Ok(())
}

// Process DeltaLake table stored in S3
// Processes the returned Arrow RecordBatch
fn process_record_batch(batch: &RecordBatch) {
    println!("Number of columns: {}", batch.num_columns());
    println!("Number of rows: {}", batch.num_rows());

    let schema = batch.schema();

    for i in 0..batch.num_columns() {
        let field = schema.field(i);

        println!(
            "Column {}: '{}' (Type: {:?})",
            i,
            field.name(),
            field.data_type()
        );
    }
}
