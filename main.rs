use colored::Colorize;
use std::io::{stdin, stdout};
use std::io::Write;
use rand::Rng;

// TODO
// 1. Get problem statement
// 2. Design:
//	-  divide and conquer
//	- ask interviewer questions
//	- ask yourself questions: ex: What are my inputs, outputs?
// 3. Write code
// Note: Rust highlights TODO by default

/*

 Design

	Magic 8-ball program, loops forever
	-------------------
	1. Takes user input:
		if  quit, exit
		if question, continue
	2. Create possible responses
	3. Generate random number
	4. Pick response based on random number
	5. Prints respose
	6. Loop

*/

fn main()
{

	//TODO Create possible responses
		//Vec(vec![response,color], vec![response, color])
		//more generic than String
	let mut responses = Vec::new();
	let response1 = vec!["Yes definitely.", "g"];
	let response2 = vec!["Ask again later.", "y"];
	let response3 = vec!["Don't count on it.", "r"];
	let response4 = vec!["Without a doubt.", "b"];
	let response5 = vec!["Concentrate and ask again.", "p"];
	let response6 = vec!["My sources say no.", "c"];
	responses.push(response1);
	responses.push(response2);
	responses.push(response3);
	responses.push(response4);
	responses.push(response5);
	responses.push(response6);

	//TODO Generate random numer
	let mut rng = rand::thread_rng();
		//let mut random_index = rng.gen_range(0..responses.len());
	
	//Output response
	let mut output;

	// TODO Prompt user
	loop {
	println!("Magic 8-ball");
	print!("Ask me a question...");
	
	//TODO Take user input
		//match std::io::stdout().flush()
	match stdout().flush()
	 {
		//result tuple handle case where everything is ok
		// and where everthing is an error
		//can do stuff to the o, take whatever is the output and forward it
	  Ok(o) => o,
	  Err(_) => {println!("Error getting stdout"); return;}
	}

	let mut input = String::new(); //String to hold user input
	let stdinput = stdin(); // standard input output
	//if something goes wrong in read line, use expect to show error
	stdinput.read_line(&mut  input).expect("Did not enter a correct string.");
	//println!();
	if input.trim() == "quit" || input.trim() == "q" ||input.trim() == "exit"
		 || input.trim() == "e" {break;}

	//Generates random number
	let random_index = rng.gen_range(0..responses.len());

	//TODO Use random number to pick a response
	//this is the wrapping Vector object
	output = match responses.get(random_index){
	  Some(o) => o,
	  None => {println!("No responses available."); return;}
		};

	//TODO Print response
	match output[1]{
	"g" => println!("{}", output[0].bright_green()),//print green 
	"r" => println!("{}", output[0].bright_red()),
	"y" => println!("{}", output[0].bright_yellow()),
	"b" => println!("{}", output[0].bright_blue()),
	"p" => println!("{}", output[0].bright_magenta()),
	"c" => println!("{}", output[0].cyan()),
	_ => {println!("Invalid response color found."); return;}

	} 
      }
}

