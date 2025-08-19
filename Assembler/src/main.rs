use owo_colors::OwoColorize;
use std::process;

mod grep;
mod files;
mod assembler;

mod parser;



use std::time::Instant;

fn main() {


    //Match results of grep function to Ok and errors
    match grep::grep(){

        Ok(filepath)=>{
        //At this point, the grep tool has successfully parsed the argument from the environment,
        // which indicates the filepath of the ASM file, so logic flows.

        let mut filecontent:  Vec<String> = Vec::new();

        let res = files::read_file(&filepath, &mut filecontent);

        //Matches result of read_file function to ok and error to continue logic flow.
        match res{
        Ok(())=>{
            //Starting Parser for the file contents
        
        let start = Instant::now();
        let machine_code_vec = assembler::execute(&mut filecontent);

        let duration = start.elapsed();
        println!("Executing time: {:?}", duration.green());

        files::write_file(&filepath, & machine_code_vec);
        }
        
        //If reading file failed then it prints error and terminate the process
        Err(e)=>{
            eprintln!("{}", e.red());
            process::exit(1);
        }
    }
        }
        
        //If grep failed then it prints error and terminate the process
        Err(e)=>{
            eprintln!("{}", e.red());
            process::exit(1);
        }
    }

}


