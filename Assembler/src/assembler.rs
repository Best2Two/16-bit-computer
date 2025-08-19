use std::{collections::HashMap, str::FromStr};
use crate::parser;
use owo_colors::OwoColorize;
use parser::parse_assembly_file;

pub fn execute(assembly_vector: &mut Vec<String>)-> Vec<String>{

    let mut machine_code_vector : Vec<String> = Vec::new();

    //Tables (Preserved Labels, User-Defined Labels, Varaiables)
    //Symmols table for the reserved symbols by the language specifications
    let PRESERVED_SYMBOLS_TABLE: HashMap<&'static str, i16>= HashMap::from(
        [
            ("R0",0x0),
            ("R1",0x1),
            ("R2",0x2),
            ("R3",0x3),
            ("R4",0x4),
            ("R5",0x5),
            ("R6",0x6),
            ("R7",0x7),
            ("R8",0x8),
            ("R9",0x9),
            ("R10",0xA),
            ("R11",0xB),
            ("R12",0xC),
            ("R13",0xD),
            ("R14",0xE),
            ("R15",0xF),
            ("SCREEN",0x4000),
            ("KBD",0x6000),
            ("SP",0x0),
            ("LCL",0x1),
            ("ARG",0x2),
            ("THIS",0x3),
            ("THAT",0x4),
        ]
    );
    //User defined
    let mut user_defined_symbols_table: HashMap<String, i16> =  HashMap::new();
    //Varaibles
    let mut variables_table: HashMap<String, i16> =  HashMap::new();

    //Instruction specified by the language

    let dest: HashMap<&str, &str> = HashMap::from([
        ("NULL", "000"),
        ("M", "001"),
        ("D", "010"),
        ("A", "100"),
        ("MD", "011"),
        ("AM", "101"),
        ("AD", "110"),
        ("AMD", "111"),
    ]);
    let comp: HashMap<&str, &str> = HashMap::from([
        ("0", "0101010"),
        ("1", "0111111"),
        ("-1", "0111010"),
        ("D", "0001100"),
        ("A", "0110000"),
        ("!D", "0001101"),
        ("!A", "0110001"),
        ("-D", "0001111"),
        ("-A", "0110011"),
        ("D+1", "0011111"),
        ("A+1", "0110111"),
        ("D-1", "0001110"),
        ("A-1", "0110010"),
        ("D+A", "0000010"),
        ("D-A", "0010011"),
        ("A-D", "0000111"),
        ("D&A", "0000000"),
        ("D|A", "0010101"),
        ("M", "1110000"),
        ("!M", "1110001"),
        ("-M", "1110011"),
        ("M+1", "1110111"),
        ("M-1", "1110010"),
        ("D+M", "1000010"),
        ("D-M", "1010011"),
        ("M-D", "1000111"),
        ("D&M", "1000000"),
        ("D|M", "1010101"),
    ]);
    let jump: HashMap<&str, &str> = HashMap::from([
        ("NULL", "000"),
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
        ("JMP", "111"),
    ]);
    
   //Parsing the assembly file it parses ascii first then parses C instructions, A instructions is parsed by symbol solvers
   //parser doesn't return errors instead it terminate process as it is part of assembler modulus
    parser::parse_assembly_file(assembly_vector, &dest, &comp, &jump);    
    //Resolving symbols, process will be terminated here if something went wrong
    _resolve_symbols_by_parser(assembly_vector, &mut user_defined_symbols_table, &PRESERVED_SYMBOLS_TABLE, &mut variables_table);
    _assemblies_to_machine( assembly_vector, &dest, &comp, &jump, &PRESERVED_SYMBOLS_TABLE, &user_defined_symbols_table, &variables_table, &mut machine_code_vector);

    //End of execution here
    machine_code_vector
    
}

fn _resolve_symbols_by_parser(assembly_vector :&mut Vec<String>,user_defined_symbols_table:&mut HashMap<String, i16>,
PRESERVED_SYMBOLS_TABLE:& HashMap<&'static str, i16>, variables_table:&mut HashMap<String, i16>){

    //parser resolved symbols if there are any errors: Double occurrences for user defined labels or blank labels it print error to the user then terminate the process
    parser::resolve_symbols(assembly_vector, user_defined_symbols_table, PRESERVED_SYMBOLS_TABLE, variables_table);
}

fn _assemblies_to_machine(assembly_vector :&mut Vec<String>,
    dest: &HashMap<&str, &str>, comp: &HashMap<&str, &str>, jump: &HashMap<&str, &str>,
    PRESERVED_SYMBOLS_TABLE: &HashMap<&'static str, i16>, user_defined_symbols_table: &HashMap<String, i16>, variables_table: &HashMap<String, i16>
    ,machine_code_vector : & mut Vec<String>){
    
    for element in assembly_vector{

        let mut machine_code:String=String::new();

        //If instruction A-instruction
        if element.starts_with("@"){
            let mut address_value=String::new();

            //neglecting leading '@'
            let addrress_instruction_value: String = String::from(&element[1..]);
            
            //if all address instruction is numeric this means it's not a label
            if addrress_instruction_value.parse::<u32>().is_ok(){
                    //address converted to numeric value then formatted to be binary string with leading 15
                    address_value = addrress_instruction_value;           
            }

            //if not then it is a label or a variable specified by the user
            else{

            //Check for labels
            if let Some(value) = PRESERVED_SYMBOLS_TABLE.get(&addrress_instruction_value.as_str()){
                address_value = value.to_string();
            }
            else if let Some(value) = user_defined_symbols_table.get(&addrress_instruction_value){
                address_value = value.to_string();
            }
            else if let Some(value) = variables_table.get(&addrress_instruction_value){
                address_value = value.to_string();
            }

            }

            let opcode = "0";
            let address = format!("{:015b}", address_value.parse::<u32>().unwrap());
            machine_code = format!("{}{}", opcode, address);
        }

        //If instruction is not an address instruction also it is not a label then it is C instruction
        //If C-instruction
        else if !element.starts_with("("){

        //Split into a vector of instruction parts dest, comp and jump
        let instruction_vector: Vec<&str> = element.split(',').collect();

        let mut dest_value_bi_string: String = String::new();
        let mut comp_value_bi_string: String = String::new();
        let mut jump_value_bi_string: String = String::new();


        if let Some(val) = comp.get(instruction_vector[1]){
            comp_value_bi_string.push_str(val);
        }
        if let Some(val) = dest.get(instruction_vector[0]){
            dest_value_bi_string.push_str(val);
        }
        if let Some(val) = jump.get(instruction_vector[2]){
            jump_value_bi_string.push_str(val);

        }
        
        let opcode =  "111";
        
        machine_code = format!("{}{}{}{}", opcode, comp_value_bi_string, dest_value_bi_string, jump_value_bi_string);        
        }

        if !element.starts_with("("){
                    machine_code_vector.push(machine_code);
        }
    }
}