use std::path::PathBuf;
use clap::Subcommand;
use nestify::nest;

nest! {
    #[derive(clap::Args, Debug, Clone)]
    pub struct ImportCommand {
        /// The type of file to import
        #[command(subcommand)]
        pub import_type: #[derive(Subcommand, Debug, Clone)] pub enum ImportType {
            /// Import a Postman v2.1.0 file
            Postman(PostmanImport),

            /// Import a Postman environment file
            PostmanEnv(PostmanEnvImport),

            /// Import a curl file
            Curl(CurlImport),

            /// Import an Open Api file
            #[clap(alias = "openapi")]
            OpenApi(OpenApiImport)
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
pub struct PostmanImport {
    /// Path to the file to import
    #[clap(value_hint = clap::ValueHint::FilePath)]
    pub import_path: PathBuf,

    /// Max depth at which import should stop creating nested collections and only get the deeper requests
    #[arg(long)]
    pub max_depth: Option<u16>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct PostmanEnvImport {
    /// Path to the file to import
    #[clap(value_hint = clap::ValueHint::FilePath)]
    pub import_path: PathBuf,

    /// Make each variable key uppercase
    #[clap(short, long, default_value_t = false)]
    pub force_uppercase_keys: bool,

    /// Also retrieve disabled variables
    #[clap(short, long, default_value_t = false)]
    pub use_disabled: bool,
}

#[derive(clap::Args, Debug, Clone)]
pub struct CurlImport {
    /// Path to the file/folder to import
    #[clap(value_hint = clap::ValueHint::AnyPath)]
    pub import_path: PathBuf,

    /// Collection name to save the request to
    pub collection_name: String,

    /// Request name (will use the file name if none is provided)
    pub request_name: Option<String>,

    /// Search for deeper files
    #[arg(short, long, conflicts_with = "request_name")]
    pub recursive: bool,

    /// Max depth at which import should stop creating nested collections and only get the deeper requests
    #[arg(long, requires = "recursive", conflicts_with = "request_name")]
    pub max_depth: Option<u16>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct OpenApiImport {
    /// Path to the file to import
    #[clap(value_hint = clap::ValueHint::FilePath)]
    pub import_path: PathBuf,

    /// Max depth at which import should stop creating nested collections and only get the deeper requests
    #[arg(long)]
    pub max_depth: Option<u16>,
}