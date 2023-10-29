pub mod mass_move;
use clap::Parser;

// #[derive(Parser, Debug)]                                                                               
// pub struct Args {
//     /// file template                                                                                 
//     pub source_pattern: String,  
//     /// marker file
//     pub target_pattern: String,                                                                        
//     #[arg(short, long)]                                                                               
//     pub force: bool,                                                                                   
// }

fn main(){
    // let args = Args::parse();
    mass_move::mass_moving("/home/sekigo/Check/some_*_filename.*".to_string(),
    "/home/sekigo/new_place/changed_#1_filename.#2".to_string());
}
