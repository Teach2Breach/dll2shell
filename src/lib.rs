use std::fs::File;
use std::io::Write;
pub mod shellcode;

    pub fn gen_shellcode (args: Vec<String>) {   // create shellcode and output to file

        //println!("args: {:?}", args);
        let link_shellcode = shellcode::shellcode_rdi(&args[1], &args[2], "".to_string());
        let mut output_file = File::create("shellcode.bin").expect("could not write file");
        output_file
            .write_all(&link_shellcode)
            .expect("could not write contents to output file");
        println!("output: shellcode.bin");
    }
