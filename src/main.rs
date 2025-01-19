pub mod shellcode;
use dll2shell::gen_shellcode;


fn main() {
    //collect command line args
    let args: Vec<String> = std::env::args().collect();

    //make sure we got at least 2 positional arguments
    if args.len() < 2 {
        println!("Usage: dll2shell.exe <payload.dll> <exported_function>");
        return;
    }
    
    gen_shellcode(args);
}
