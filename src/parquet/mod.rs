use datafusion::prelude::*;
use colored::Colorize;

pub struct ParquetFile {  
    path: String,
}

//TODO: add in remote parquet file support
//TODO: add extra impls and structs for other file types
impl ParquetFile {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
    
    pub async fn display_basic_info(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new session context
        let ctx = SessionContext::new();
        
        // Register the parquet file as a table
        ctx.register_parquet("temp_table", &self.path, ParquetReadOptions::default()).await?;
        
        // Get the dataframe
        let df = ctx.table("temp_table").await?;
        let schema = df.schema();
    
        // Print basic file information
        println!("{}", "\nParquet File Information:".cyan());
        
        // Get row count
        let count_df = ctx.sql("SELECT COUNT(*) as row_count FROM temp_table").await?;
        let count = count_df.collect().await?;
        println!("Total number of rows: {:?}", count);
        println!("Number of columns: {}", schema.fields().len());
    
        // Print schema information
        println!("\n{}", "Schema:".cyan());
        println!("{}", schema);
    
        // Print sample data
        println!("\n{}", "Sample Data (First 5 rows):".cyan());
        let sample_df = ctx.sql("SELECT * FROM temp_table LIMIT 5").await?;
        let results = sample_df.collect().await?;
        for batch in results {
            println!("{:?}", batch);
        }
    
        // Print basic statistics
        println!("\n{}", "Basic Statistics:".cyan());
        let stats_sql = schema
            .fields()
            .iter()
            .filter(|field| field.data_type().is_numeric())
            .map(|field| {
                format!(
                    "MIN({0}) as min_{0}, 
                    MAX({0}) as max_{0}, 
                    AVG({0}) as avg_{0}",
                    field.name()
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
    
        if !stats_sql.is_empty() {
            let stats_df = ctx.sql(&format!("SELECT {} FROM temp_table", stats_sql)).await?;
            println!("\nNumerical Columns Statistics:");
            let results = stats_df.collect().await?;
            for batch in results {
                println!("{:?}", batch);
            }
        }
    
        // Print null counts
        let null_sql = schema
            .fields()
            .iter()
            .map(|field| {
                format!(
                    "COUNT(CASE WHEN {} IS NULL THEN 1 END) as null_count_{}",
                    field.name(),
                    field.name()
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
    
        let null_df = ctx.sql(&format!("SELECT {} FROM temp_table", null_sql)).await?;
        println!("\nNull Counts:");
        let results = null_df.collect().await?;
        for batch in results {
            println!("{:?}", batch);
        }
    
        Ok(())
    }
    
        
}

