use std::env;

fn verbose() {
    let name = "USER";
    match env::var(name) {
        Ok(v) => println!("{}: {}", name, v),
        Err(e) => panic!("${} is not set ({})", name, e),
    }
}

fn short() {
    let v = env::var("USER").expect("$USER is not set");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut company: String = "APPL".to_string();

    if args.len() < 2 {
        println!("No symbol has been specified, so using AAPL as default");
    }
    else {
        company = args[1].clone();
    }
    println!("{:?}", args);    
    println!("Hello, world!");
    println!("{}", company);
    
    
}
