mod clap;

use ::clap::Parser;
use clap::Args;

fn main() {
    let args = Args::parse();

   if args.verbose {
        println!("Input:   {}", args.input);
        println!("Output:  {:?}", args.output);
        println!("Threads: {}", args.threads);
    
   }
}           