use std::env;
use std::error::Error;


pub fn grep() -> Result<String, Box<dyn Error>> {

    //Our grep tool is only interested in the first argument only considering it as a file, other arguments
    //are necessarily neglected

    match env::args().nth(1)
    {
        Some(filepath) => Ok(filepath),
        //User specified error because main error is ambiguous to the end-user in this situation
        None => Err("Failed parsing argument the assembly file path is not passed as an argument!".into()),
    }
}
