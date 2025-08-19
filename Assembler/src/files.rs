
use std::fs;
use std::error::Error;
use std::path::Path;

//ToDo: This function takes filepath as first parameter and borrows a filecontent variable assigned by the user to be overwritten
pub fn read_file(filepath: &String, filecontent: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
   let content = fs::read_to_string(filepath)?;
   *filecontent = content.lines().map(|line| line.to_string()).collect();
   Ok(())
}



pub fn write_file(path: &String, contents: & Vec<String>){

   //Files are closed automatically when they are out of scope, I don't know what is meant by out of scope but I guess it means
   //when the context [Task] or [Thread] are switched, however I will process the vector to string then push it line by line


   //this is not the best approach of course as the file contents will be loaded into the memory twice and this is a huge
   //wasting for the memory, I will use buffers here with proper writing function, but I will let it like this for now
   let text = contents.join("\n");
   let filename = Path::new(path).file_name().unwrap().to_str().unwrap();
   let bin_filename = format!("{}.bin", filename);
   fs::write(bin_filename, text);

}


// pub fn write_file(filepath: String, filecontent: &mut String){
    
// }