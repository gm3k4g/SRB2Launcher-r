// main -- error handling

fn main() {
    match srb2launcher_r::run() {
    	Err(e) => {
    		println!("Application error: {}", e);
    		std::process::exit(1);
    	}
    	_ => { //println!("Application exit normal");
    	},
    };
}