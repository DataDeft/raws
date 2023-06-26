use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    service: Services,
}

#[derive(Subcommand, Debug)]
enum Services {
    /// S3
    S3 { command: S3Commands },
}

#[derive(Subcommand, Debug, Clone)]
enum S3Commands {
    LS { params: String}
}



// pub async fn list_objects(client: &Client, bucket_name: &str) -> Result<(), Error> {
//     let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
//     println!("Objects in bucket:");
//     for obj in objects.contents().unwrap_or_default() {
//         println!("{:?}", obj.key().unwrap());
//     }

//     Ok(())
// }

fn main() {
    let cli = Cli::parse();
    println!("Hello, world! {:?}", cli);
    match &cli.service {
        Services::S3 { command } => {
            println!("'myapp s3' was used, name is: {command:?}")
        }
    }
}
