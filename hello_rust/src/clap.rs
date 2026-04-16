use clap::Parser;


/// A simple file processor — this doc comment becomes the help text
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
pub struct Args {
    
    /// Input file to process
    #[arg(short, long)]
    pub input: String,
    /// Output file (defaults to stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Number of worker threads
    #[arg(short = 'j', long, default_value_t = 4)]
    pub threads: usize,
}  

