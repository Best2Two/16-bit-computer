use std::error::Error;
use std::{collections::HashMap};
use std::process;
use owo_colors::OwoColorize;


// WARNING: Error handling in this file is intentionally spaghetti code.
// This is technical debt that needs to be refactored into a proper error handling module, Also values i16 or usize should be fixed propeply
// TODO: Create dedicated error handling system.


//Utility
fn _convert_instruction(input: String) -> String {
    let parts: Vec<&str> = input.split(',')
        .filter(|&part| part != "NULL")
        .collect();
    
    if parts.len() >= 2 {
        // Join with = for first separator and ; for last separator
        if parts.len() == 2 {
            format!("{}={}", parts[0], parts[1])
        } else {
            let mut result = parts[0].to_string();
            result.push('=');
            
            // Add middle parts with commas
            for i in 1..parts.len()-1 {
                result.push_str(parts[i]);
                result.push(',');
            }
            
            // Add last part with semicolon
            result.push_str(parts[parts.len()-1]);
            
            // Replace the last comma before the last part with semicolon
            let last_comma_pos = result.rfind(',').unwrap();
            result.replace_range(last_comma_pos..last_comma_pos+1, ";");
            
            result
        }
    } else {
        // If less than 2 non-NULL parts, just remove NULLs
        parts.join(",")
    }
}


//Parsers
pub fn parse_assembly_file(filecontent: &mut Vec<String>,
    dest_table: &HashMap<&str, &str>, 
    comp_table: &HashMap<&str, &str>,
    jmp_table: &HashMap<&str, &str>){
    
    
    //Parse Ascii and syntax errors
    _parse_file_syntax(filecontent);
    //Parse C Instructions
    _parse_c_instructions(filecontent, &dest_table, &comp_table, &jmp_table);
}

fn _parse_file_syntax(filecontent: &mut Vec<String>){

    let delim =",";

   let mut filtered_assembly_vector : Vec<String>= Vec::new();

   for (i, line) in filecontent.iter_mut().enumerate() {

    let mut filtered_line = line.to_string();

    if !line.is_ascii(){
        eprint!("{} {}","Error: Assembly file can not contain non-ascii charachter, found one in line".red(),(i+1).to_string().yellow());
        process::exit(1);
    }   

    //Search for comment specifier
    if let Some(pos) = filtered_line.find("//"){
    filtered_line.truncate(pos); // Keeps everything before pos
    }
    filtered_line.retain(|c| !c.is_whitespace());

    if !filtered_line.is_empty(){
        
        //Check for delimeter it can trick the other error handlers, again this is bad approach and need serious fixing
        if let Some(coma)=filtered_line.find(','){
            eprintln!("{} {} {} {} {}","Error instruction".red(), filtered_line.yellow() ,"at line".red(),i.to_string().yellow(),"cannot be a valid instruction".red());
            process::exit(1);
        }
        else{
        filtered_assembly_vector.push(filtered_line);
        }
    }
   };

   if filtered_assembly_vector.is_empty(){
        eprintln!("{}", "Error assembly file can't be empty".red());
        process::exit(1)
   }
   *filecontent = filtered_assembly_vector

}

fn _parse_c_instructions(

    assembly_vector :&mut Vec<String>,
    dest_table: &HashMap<&str, &str>, 
    comp_table: &HashMap<&str, &str>,
    jmp_table: &HashMap<&str, &str>){

    for (i, element) in assembly_vector.iter_mut().enumerate(){

        //if c instruction then we need to parse instruction using the delimeter ,
        if(!element.starts_with("@") && !element.starts_with("(")){
            
            let delim: &str = ",";    

            let mut comp_indx: i8=-1;
            let mut comp_end_indx: i8 =-1;

            let mut dest_indx: i8=-1;
            let mut jmp_indx: i8=-1;


            let mut dest_instruction : String= String::new();
            let mut comp_instruction : String= String::new();
            let mut jmp_instruction : String= String::new();
                      
            //check for destination bits           
            let dest_rest = element.find("=");

            //Check for = because destination is always before =
            match dest_rest {
            Some(dst_start) => {
                
                dest_indx = dst_start as i8;

                let dest_inst = &element[0..dst_start];

                match dest_table.get(dest_inst) {
                Some(value) => {
                dest_instruction.push_str(dest_inst);
                },
                None => {
                    let dest_str = dest_inst.to_string();
                    eprint!("{} {} {} {} {}","Error: Instruction".red(),_convert_instruction(assembly_vector[i].to_string()).yellow(),"is invalid".red(),dest_str.yellow(),"cannot be a destenation".red());
                    process::exit(1);
                }
                }
            },

            None => {
                dest_instruction.push_str("NULL");
            }
        };

           //Check for Jump instruction
            let jmp_rest = element.find(";");
            match jmp_rest {
            Some(jmp_start) => {

                jmp_indx = jmp_start as i8 + 1;

                let jmp_inst = &element[jmp_indx as usize..];

                match jmp_table.get(jmp_inst) {
                Some(value) => {
                jmp_instruction.push_str(jmp_inst);
                },
                None => {
                let jump_str = jmp_inst.to_string();
                eprint!("{} {} {} {} {}","Error: Instruction".red(),_convert_instruction(assembly_vector[i].to_string()).yellow(),"is invalid".red(),jump_str.yellow(),"cannot be a jump".red());

                    process::exit(1);
                }
                }
            },          
            None => {
                jmp_instruction.push_str("NULL");
                }

            };


            //Computation instruction starts after '=' is found and before ';' is found

            //if there is no destination or computation instructions
            
            //if there is no destination then bit start from 0
            if dest_indx == -1{
                comp_indx = 0
            }
            //if there is destination then bit start from destination +1
            else{
                comp_indx = dest_indx+1;
            }

            //if there is no jmp then end index is len
            if jmp_indx == -1{
                comp_end_indx = element.len() as i8;
            }
            //if there is jmp instruction then end of comp is start of jmp
            else{
                comp_end_indx = jmp_indx-1;
            }

            
                let comp_inst = &element[comp_indx as usize ..comp_end_indx as usize];

                match comp_table.get(comp_inst) {
                Some(value) => {
                comp_instruction.push_str(comp_inst);
                },

                None => {                  

                    let comp_str = comp_inst.to_string();
                    eprint!("{} {} {} {} {}","Error: Instruction".red(),_convert_instruction(assembly_vector[i].to_string()).yellow(),"is invalid".red(),comp_str.yellow(),"cannot be a computation".red());

                    process::exit(1);
                }
                }
                        
            let c_instruction = dest_instruction + delim + &comp_instruction + delim + &jmp_instruction;  
            *element = c_instruction;
    }

}
    }
//Resolvers

pub fn resolve_symbols(assembly_vector :&mut Vec<String>,
    user_defined_symbols_table:&mut HashMap<String, i16>,
    READ_ONLY_SYMBOLS_TABLE:& HashMap<&'static str, i16>,
    variables_table:&mut HashMap<String, i16>){
    
    //First path resolving for user defined labels
    _first_path_resolve(&assembly_vector,user_defined_symbols_table);

    //Second path resolving for the variables
    _second_path_resolve(&assembly_vector,user_defined_symbols_table, READ_ONLY_SYMBOLS_TABLE
    ,variables_table);
}
//First path to resolve user-defined label symbols
fn _first_path_resolve(assembly_vector: &Vec<String>, symbol_table: &mut HashMap<String, i16>) {
    
    for (i, element) in assembly_vector.iter().enumerate() {
        //User defined labels starts with '(' in language spesifications
        if(element.starts_with("(")){
            //Remove () spesification
            let mut key=element.clone();
            key.remove(0);
            key.remove(key.len()-1);  
            
            if key==""{

                eprintln!("{} {} {}","Error: label for instruction".red(),_convert_instruction(assembly_vector[i+1].to_string()).yellow(),"cannot be blank or empty near instruction".red());
                process::exit(1);
            }
            else if symbol_table.get(&key) == None{
                
                symbol_table.insert(key, i as i16 +1);
            }
            else{
                eprintln!("{} {} {}","Error more than one occurence for label".red(), key.to_string().yellow(),"was found".red());
                process::exit(1);
            }
        }
    }
}

//For seperation of concerns this is a second path resolve to resolve all variables in the file
fn _second_path_resolve(assembly_vector: &Vec<String>, user_defined_symbol_table: &mut HashMap<String, i16>,
    READ_ONLY_SYMBOLS_TABLE:& HashMap<&'static str, i16>, variables_table: &mut HashMap<String, i16>){

    let mut variables_counter= 17;
     for element in assembly_vector {
        if(element.starts_with("@")){      
            //Remove @ spesification
            let mut key=element.clone();
            key.remove(0);
            if user_defined_symbol_table.get(&key) == None && 
            READ_ONLY_SYMBOLS_TABLE.get(&key.as_str())==None
            && variables_table.get(&key)==None {
                variables_table.insert(key, variables_counter);
                variables_counter+=1;               
            }
        }
    }
}
