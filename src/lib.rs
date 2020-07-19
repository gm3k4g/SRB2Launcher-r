// lib -- core logic of program

use std::fs::OpenOptions;
use std::process::Command;

use std::io::Write;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
	// IP address to connect to
	let mut ip: std::string::String = std::string::String::new();
	// application arguments
	let mut args: std::string::String = std::string::String::new();
	// application name
	let mut application = std::string::String::from("srb2");

	// first time opening?
	let mut firsttime = true;

	// assignments
	args = "-skipintro".to_string();
	ip = "localhost".to_string();

	// file to open and save IPs into
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open("srb2l-r_list.txt");

	// check if this is not the first time opening srb2l-r
	// if list.contains_var then firsttime = false;

	// Client to make multiple requests with
	let client = reqwest::blocking::Client::new();

	// update body
	let body = update_list(client).unwrap();
	println!("{}",body);
	
	// Input variable
	let mut input: std::string::String = std::string::String::new();


	// starting message: clear terminal and show this
	println!("{}[2J", 27 as char);
	println!("========================== SRB2 LAUNCHER-r =====================");

	// start loop
	'running: loop {
		
		if firsttime {
			println!("NOTICE: If this is your first time using SRB2 Launcher-r, type \"help\" without\n the quotation marks for more details. This message will not appear again.");
			// disable firsttime
			firsttime = false;
			// write variable in the .txt file to make program remember
		}
		// refresh user input and await for new input
		input = std::string::String::new();
		print!("launcher$: ");
		std::io::stdout().flush().unwrap();
		std::io::stdin().read_line(&mut input).expect("Error getting input");
		input.pop();

		// match string whose trailing whitespace is truncated
		match input.trim_end() {
			"help" => {
				println!("	help: 	     Show this message.");
				println!("	update:      Update server list and display available servers,");
				println!("			     	in realtime. (??)");
				println!("	list: 	     Update server list file and display available servers.");
				println!("			     	If getting servers fails or there is no response,");
				println!("			     	the available list file is used to display servers.");
				println!("	connect: 	 Connect to a specified IP.");
				println!(" 				 	i.e. \"connect 192.168.0.1\"");
				println!("	run: 	     Run SRB2 with the current commandline arguments,");
				println!("			     	without connecting to any server.");
				println!("	args: 	     Change SRB2 commandline arguments.");
				println!("	version: 	 Show the current version of the application.");
				println!("	exit | quit: Exit application");
			},
			"list" => {
				println!("{}[2J", 27 as char);
				println!("================ AVAILABLE SERVERS....");
				// let i = 1;
				//for ip in list.txt
				// println!("{}. {}",i,ip);
				// i++
			},
			"run" => {
				println!("STARTING SRB2...");
				exec_srb2(&application, &args);
			},
			"clear" => {
				println!("{}[2J", 27 as char);
			}
			"exit" | "quit" => {
				println!("Quitting..");
				break 'running;
			},
			"" => {},
			_ => { println!("Invalid command"); },
		};
	}
    Ok(())
}


pub fn exec_srb2(application: &std::string::String, args: &std::string::String) {
	println!("STARTING SRB2...");
    let start = Command::new(application)
                      .arg(args)
                      .output()
                      .unwrap();
}

pub fn update_list(client: reqwest::blocking::Client) -> Result<std::string::String, ()> {
	let body: std::string::String = client.get("https://mb.srb2.org/masterserver.php").send()
		.unwrap()
		.text()
		.unwrap_or("".to_string());

	/*if let Ok(resp) = client.get("https://mb.srb2.org/masterserver.php").send() {
		let body = resp.text();
		// body = get_ips(body);
		//println!("body = {:?}", body);
		println!("List updated");
		return Some(body);
	}*/

	Ok((body))
}



/*pub fn mainloop () -> Result<(), Box<dyn std::error::Error>> {

}*/