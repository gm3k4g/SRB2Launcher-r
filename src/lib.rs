// lib -- core logic of program
use std::io::Write;
use std::process::Command;
use std::collections::HashMap;
use std::time::Duration;
use std::fs::OpenOptions;

use reqwest::ClientBuilder;
use std::error::Error;

use tokio::task;

const FILENAME: &str = "srb2l-r_list.txt";

pub struct Server {
   pub ip: std::string::String,
   pub port: std::string::String, //TODO: maybe integer?
   pub name: std::string::String,
   pub version: std::string::String,
   pub selection: u8 // Number selection corresponding to the server
   // TODO: implement the following (create ASKINFO packet for these?)
   //gametype:
   //capacity:
   //files:
   //ping:
}
impl std::fmt::Display for Server {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Name: {}, Version: {}\n    IP Address: {}:{}",self.name, self.version, self.ip, self.port)
	}
}

pub fn load_servers(servers: &mut Vec<Server>) -> Result<(), Box<dyn std::error::Error>> {
	// Create runtime
	let mut runtime = tokio::runtime::Builder::new()
	    .basic_scheduler()
	    .threaded_scheduler()
	    .enable_all()
	    .build()
	    .unwrap();
	// Spawn the runtime and run async fn
	runtime
	    .block_on(async_load_servers(servers))
}

async fn async_load_servers(servers: &mut Vec<Server>) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("https://mb.srb2.org/MS/0/servers");
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build().unwrap();
    //println!("GET_response");
    let mut resp = client.get(&request_url)
    	.send()
    	.await?
    	.text()
    	.await?;

    let string = resp.to_string();
    let mut words = string.split('\n');
    let mut ip: std::string::String;
    let mut port: std::string::String;
    let mut name: std::string::String;
    let mut version: std::string::String;

    let mut listing: u8 = 1;
    while let Some(word) = words.next() {
    	// ignore room numbers
    	if word.len() <= 2 {
    		continue;
    	}

    	let mut server = word.split(' ');
    	for sip in server.next() {
    		ip = sip.to_string();
    		port 	= server.next().or(Some("")).unwrap().to_string();
    		name 	= server.next().or(Some("")).unwrap().to_string();
    		version = server.next().or(Some("")).unwrap().to_string();
    		server.next().or(Some("")).unwrap();
    		server.next().or(Some("")).unwrap();
    		servers.push(
    			Server {
	    			ip: ip,
	    			port: port,
	    			name: name,
	    			version: version,
	    			selection: listing
    		});
    	}
    	listing += 1;
    }
    Ok(())
}


// TODO: Creation of structs for better manipulation/cleaner code
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
	// IP address to connect to
	let mut ip = Some(std::string::String::new());
	// application arguments
	let mut args: Vec<std::string::String> = Vec::new(); //std::string::String::new(); // TODO: read from file
	// application name
	let mut application = std::string::String::from("srb2"); // TODO: read from file
	// first time opening?
	let mut firsttime = true; // TODO: read from file
	// was list used?
	let mut listused = false;
	// have we joined any server yet?
	let mut joined = false;
	// Last joined server
	let mut last = std::string::String::new();

	// TODO: read args from list. read IP from `list` command
	// assignments
	//args = "-skipintro".to_string();
	//ip = "localhost".to_string();

	// server list file to open and save IPs into
	let mut serv_list = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(FILENAME)
		.unwrap();

	// Will contain all our servers
	let mut servers: Vec<Server> = Vec::new();
	// Get servers
	load_servers(&mut servers);

	// check if this is not the first time opening srb2l-r
	// if list.contains_var then firsttime = false;
	
	// Input variable
	let mut input: std::string::String = std::string::String::new();

	// TODO: create option to allow immediately displaying servers upon startup?
	println!("{}[2J", 27 as char);
	println!("========================== SRB2 LAUNCHER-r =====================");

	if firsttime {
		println!("NOTICE: If this is your first time using SRB2 Launcher-r, type \"help\" without\n the quotation marks for more details. This message will not appear again.");
		firsttime = false; // TODO: write variable in the .txt file to make program remember
	}

	// start loop
	'running: loop {
		list_servers(&mut servers, &mut listused);
		
		// refresh user input and await for new input
		input = std::string::String::new();
		print!("launcher$: ");
		std::io::stdout().flush().unwrap();
		std::io::stdin().read_line(&mut input).expect("Error getting input");
		input.pop();

		// If 'list' was typed
		if listused {
			for server in &servers {
				if server.selection == input.parse::<u8>().unwrap() {
					println!("Jumping into server...");
					let ip = format!("{}:{}", server.ip, server.port);
					last = ip.clone();
					push_ip(&mut args, ip);
					exec_srb2(&application, &mut args, &None);
					pop_ip(&mut args);
					// Reset listused variable
					//listused = false;
					// We have currently joined a server
					joined = true;
					// Clean the console and re-iterate
					continue;
				}
			}
		}

		// match string whose trailing whitespace is truncated
		match input.trim_end() {
			"help" => {
				println!("	help: 	     	Show this message.");
				println!("	update:      	Update server list and display available servers,");
				println!("			     		in realtime. (??)");
				println!("	list: 	     	Show available servers, and update lists file.");
				println!("			     		If getting servers fails or there is no response,");
				println!("			     		the available list file is used to display servers.");
				println!("  [number]: 		Join to specified server. command list must be ran");
				println!("						beforehand in order for this to work!");
				println!("  last: 			Join the last joined server.");
				println!("	connect: 	 	Connect to a specified IP.");
				println!(" 				 		i.e. \"connect 192.168.0.1\"");
				println!("	run: 	    	 Run SRB2 with the current commandline arguments,");
				println!("			     		without connecting to any server.");
				println!("	version: 	 	Show the current version of the application.");
				println!("	options: 	 	Display some options you can enable/disable to work");
				println!(" 				 		i.e. every time the launcher starts, etc.");
				println!("	exit | quit: Exit application");
			},
			"arg_test" => {
				println!("[APPLICATION] [ARGS]: {}{:?}", application, args);
			},
			"push_ip" => {
				/*
				println!("{}[2J", 27 as char);
				input = std::string::String::new();
				println!("Testing fn push_ip:");
				println!("Pass an IP... ")
				std::io::stdout().flush().unwrap();
				std::io::stdin().read_line(&mut input).expect("Error getting input");
				input.pop();*/
				println!("Args: {:?}", args);
			},
			/*"print" => {
				// save contents of body into list
				// and then print the list
				println!("Reading servers..");
				// TODO: use update_list fn here
				// 1. Read XHR's in some variable(s)
				// 2. Store relevant data in variable(s)
				//     i.e. IP, server name, player capacity, Modified/Cheats, ping.
				//  (TODO: maybe allow for sorting/filtering?)
				// 3. Write it to list file
				//     Should follow a format such as: (to get correct format, look at MS?)
				// ============== AVAILABLE SERVERS: 
				//		 	N. [VERSION] [SERVERNAME] [CAPACITY] [MODIFIED/CHEATS] [PING]
								//[IP]
				body = update_list(&client).unwrap();
				write!(&mut serv_list, "{}", body).expect("Error writing to list file");
				println!("Write to file succesful..");
				let mut content = std::fs::read_to_string(FILENAME).expect("Error reading file");

				//serv_list.read_to_string(&mut content).expect("Error reading list file");
				println!("Contents: \n{}", content);

			}*/
			// TODO: filtering options
			// List available servers
			"list" => {
				list_servers(&mut servers, &mut listused);
			},
			// Join last joined server
			"last" => {
				match joined {
					true => {
						push_ip(&mut args, last.clone());
						exec_srb2(&application, &mut args, &None);
						pop_ip(&mut args);
					},
					false => {
						println!("ERROR: A server was not joined yet!");
					}
					_ => {},
				};
			}
			"connect" => {
					println!("{}[2J", 27 as char);
					input = std::string::String::new();
					println!("	WARNING: If you provide an invalid IP,");
					println!("		you will not be connected to the given server!");
					print!("Connecting to IPv4(:port) : ");
					std::io::stdout().flush().unwrap();
					std::io::stdin().read_line(&mut input).expect("Error getting input");
					input.pop();

					// call push_ip
					push_ip(&mut args, input);

					//ip = Some(editable);
					exec_srb2(&application, &mut args, &ip);
					// pop off "-connect" "[ip]" afterwards?
					pop_ip(&mut args);
					//ip = Some(std::string::String::new());
			}
			"run" => {
				println!("STARTING SRB2...");
				exec_srb2(&application, &mut args, &None);
			},
			// todo: most likely move options outside
			"options" => {
				println!("	Available options:");
				println!("		help: 				   Show this message.");
				println!("		startup_list [yes/no]: Grab servers upon starting this application.");
				println!(" 							   		This will also keep happening when you");
				println!("									leave the game.");
				println!("		args: 	     		   Change SRB2 commandline arguments.");
				println!("		exit: 				   Return back to launcher.");
				'inner: loop {
					// refresh user input and await for new input
					input = std::string::String::new();
					print!("options$ ");
					std::io::stdout().flush().unwrap();
					std::io::stdin().read_line(&mut input).expect("Error getting input");
					input.pop();

					match input.trim_end() {
						"help" => {
							println!("		help: 				   Show this message.");
							println!("		startup_list [yes/no]: Grab servers upon starting this application.");
							println!("		args: 	       		   Change SRB2 commandline arguments.");
							println!("		exit: 				   Return back to launcher.");
						},
						"executable" => {
							println!("Current executable: {}", application);
							// refresh user input and await for new input
							input = std::string::String::new();
							println!("NOTE: To use an executable within the current directory,");
							println!("	pre-pend the executable's name with \"./\", without the");
							println!("	quotation marks.");
							println!("i.e. if you have srb2win.exe in your current directory, you'd write");
							println!("		./srb2win.exe");
							print!("	New executable name:");
							std::io::stdout().flush().unwrap();
							std::io::stdin().read_line(&mut input).expect("Error getting input");
							input.pop();

							application = input;
							println!("Success: Your input was: {}", application);
						}
						"args" => {
							println!("Current commandline arguments: {:?}", args);
							// refresh user input and await for new input
							input = std::string::String::new();
							print!("New commandline arguments:");
							std::io::stdout().flush().unwrap();
							std::io::stdin().read_line(&mut input).expect("Error getting input");
							input.pop();

							match input.trim_end() {
								"" => {
									println!("Current commandline arguments: {:?}", args);
								},
								_ => {
									// prepend a space to all the given args
									let mut starter = std::string::String::from(" ");
									starter.push_str(input.as_str());

									args.push(input);
									println!("Success: Your input was: {:?}", args)
								},
							};

						}
						"exit" => {
							break 'inner;
						},
						"" => {},
						_ => { println!("Invalid command"); },
					};
				}
			}
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

// TODO: filtering options?
// i.e. "what to filter for? version? ping? 
pub fn list_servers(servers: &mut Vec<Server>, listused: &mut bool) -> Result<(), Box<dyn std::error::Error>> {
	println!("{}[2J", 27 as char);
	println!("================ AVAILABLE SERVERS....");
	for server in servers {
		/*if server.version != "2.2.6".to_string() {
			continue;
		}*/
		println!("{}. {}",server.selection, server);
	}
	*listused = true;
	Ok(())
}

pub fn push_ip(
	args: &mut Vec<std::string::String>, 
	ip: std::string::String) { 
	// push '-connect' argument for connecting
	args.push("-connect".to_string());
	// pass the ip after
	args.push(ip);
}

pub fn pop_ip (args: &mut Vec<std::string::String>) {
	args.pop();
	args.pop();
}

pub fn exec_srb2(
	application: &std::string::String, 
	args: &mut Vec<std::string::String>,
	ip: &Option<std::string::String>,
	) {

	match ip {
		Some(ip) => args.push(ip.to_string()),
		None => {},
		_ => {},
	};

	//let mut combination = std::string::String::new();
	println!("DEBUG: {}{:?}", application, &args);

	// run process, pass args to it
    let mut start = Command::new(application)
    				  .stdin(std::process::Stdio::inherit()) // to prevent 'stdin not a tty' error
                      .args(args) // dynamic array, contain string literals (arguments) seperated
                      .output() //.spawn() //.output()
                      .expect("Failed to run");
    //output; log, etc.
    std::io::stdout().write_all(&start.stdout).unwrap();
	std::io::stderr().write_all(&start.stderr).unwrap();
    println!("status: {}", start.status);
    println!(" Quit SRB2 succesfully..");

    // pop off ip
    /*match ip {
    	None => {},
		Some(ip) => args.pop().unwrap(),
		_ => {},
	};*/
/*
    println!("status: {}", start.status);
	println!("stdout: {}", String::from_utf8_lossy(&start.stdout));
	println!("stderr: {}", String::from_utf8_lossy(&start.stderr));
	*/
}




// this MIGHT had been used to create servers... but masochism
// left here for learning purposes
/*

	let mut document = Document::from_read(resp.as_bytes()).unwrap();

	println!("GET_server_ids");
    let mut sids = Vec::new();
    for sid in document.find(Name("tr"))
    {
        match sid.attr("id") {
            Some(attr) if !attr.starts_with("server_") => continue,
            Some(attr) => sids.push(attr.to_string()),
            None => continue,
            _ => {},
        };
    }

    println!("server_ids_to_XHR_urls");
    // Convert server IDs into XHR urls
    for i in 0..sids.len() {
    	sids[i] = sids[i].replace("server_", "");
    	sids[i] = format!("https://mb.srb2.org/ms_ajax.php?do=row&sid={}&alt=1", sids[i]);
    }

    println!("Getting server info via XHR urls..");
    // Acquire server information using the XHR urls
    let mut server_details: Vec<std::string::String> = Vec::new();
    for sid in sids {
   		resp = client.get(&sid)
    		.send()
    		.await?
    		.text()
    		.await?;
    	
    	document = Document::from_read(resp.as_bytes()).unwrap();
    	for node in document.find(Text) {
    		println!("{:?}", node);
    		println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");	
    	}
    	/*
    	let mut tds = document
        	.find(Name("td"))
        	.skip(5);
        let ctf = tds.next().unwrap().text();
        println!("{}", ctf);
        let mut tds = tds.skip(1);
        let num = tds.next().unwrap().text();
        println!("{}", num);*/
    	//servers.push(ctf,);
    	//println!("{:?}", servers);
    }
*/