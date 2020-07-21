# SRB2 Launcher-r
(The r stands for rust, most likely..)

## What is this?
This is a commandline utility (for now) to allow you to quickly connect to servers online, or just to start SRB2 with arguments of your own choice. There's still a lot of WIP, and it will probably take a while to get this complete.

## Goals
So far, there are specific goals that I have in mind in order to make this work.

== TODO's ==
1. ~~Connect to IPs~~ DONE

2. Organize code and create structures

3. List available servers by parsing HTML / Requests
	* Save the servers in a list file, where they'll be accessible just in case something goes wrong with the site.
	* Give the user the option to type in a number which corresponds to a server, and they will immediately join the specified server upon typing the number and pressing enter.
	* Give the user the option to sort available servers by ping, player capacity, game type, etc.
	* Once the user quits the game, the commandline utility will immediately refresh available servers and display them to the user.  

4. Various options, such as:
	* Immediately list servers upon start up of this program
	* Configurable commandline arguments to pass to the game upon startup
	* Config files to execute upon starting the game
	* Wads to add upon starting the game 
	* (TODO: there's probably gonna be more here)
All of which will be stored in the list file, which the program will look into to set its options correctly.

5. Utilize a GUI to implement GUI mode, while still having the option to use the commandline if the user desires to.
	* GUI will become the default mode of the program.

(TODO: there's probably gonna be more TODO's in here..)

## Testing?

So far, this is being made/has been tested only on an Arch linux x86_64.

## License

This project is under the MIT license, for more details check out the `LICENSE` file.
