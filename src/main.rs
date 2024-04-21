use std::fs::File;
use std::io::Write;
mod shellcode;

fn main() {

    //collect command line args
    let args: Vec<String> = std::env::args().collect();

    //make sure we got at least 2 positional arguments
    if args.len() < 2 {
        println!("Usage: dll2shell.exe <payload.dll> <exported_function>");
        return;
    }
    
    fn gen_shellcode (args: Vec<String>) {   // create shellcode and output to file

        //println!("args: {:?}", args);
        let link_shellcode = shellcode::shellcode_rdi(&args[1], &args[2], "".to_string());
        let mut output_file = File::create("shellcode.bin").expect("could not write file");
        output_file
            .write_all(&link_shellcode)
            .expect("could not write contents to output file");
        println!("output: shellcode.bin");
    }

    gen_shellcode(args);

}
