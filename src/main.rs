#[macro_use]
extern crate clap;
extern crate resolv;
extern crate telnet;

use telnet::{Telnet, TelnetEvent};
use std::str;
use clap::App;
use resolv::{Resolver, Class, RecordType};
use resolv::record::MX;

fn generate_combinations(name:&str,surname:&str,domain:&str) -> Vec<String> {
    println!("Generating combinations ...");
    vec![
        format!("{}.{}@{}",name,surname,domain).to_lowercase(),
        format!("{}{}@{}",name,surname,domain).to_lowercase(),
        format!("{}@{}",name,domain).to_lowercase(),
        format!("{}{}@{}",name.chars().next().unwrap(),surname,domain).to_lowercase()
    ]
}
fn get_mx_servers(domain:&str) -> Vec<String> {
    println!("Getting MX records of {} ...",domain);
    let mut resolver = Resolver::new().unwrap();
    let mut response = resolver.query(domain.as_bytes(), Class::IN,
                                      RecordType::MX).unwrap();
    let mut mx = Vec::new();
    for answer in response.answers::<MX>() {
        mx.push(answer.data.exchange.clone());
    }
    mx
}
fn send_combinations(emails:&[String],server:&str){

    let mut telnet = Telnet::connect((server, 25), 256).expect("Couldn't connect to the server...");
    // Read banner
    telnet.read().expect("Read error");
    
    // Write helo hi
    telnet.write("helo hi\r\n".as_bytes()).expect("Read error");
    telnet.read().expect("Read error");
    
    // Set from email
    telnet.write("mail from:<a@gmail.com>\r\n".as_bytes()).expect("Read error");
    telnet.read().expect("Read error");
    
    // Set rcpt emails to test it
    for email in emails {
        telnet.write(format!("rcpt to: <{}>\r\n",email).as_bytes()).expect("Read error");
        
        match telnet.read().expect("Read error") {
            TelnetEvent::Data(buffer) => {
                // Debug: print the data buffer
                let response = str::from_utf8(&buffer).unwrap();
                if response.contains("OK") {
                    println!("{} exists!", email);
                }else{
                    println!("{} does not exist!", email);
                }
            },
            _ => {}
        }
    }
}
fn main() {
    
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let domain = matches.value_of("domain").unwrap();
    let mx = get_mx_servers(&domain);
    let combos = generate_combinations(matches.value_of("name").unwrap(),matches.value_of("surname").unwrap(),&domain);
    
    println!("Testing: {:?}",combos);
    // We can only try combinations per connection
    for emails in combos.chunks(3) {
        // Get the first server or move to another if there is any error
        let server = mx.get(0);
        // If there is no more servers exit program
        if server.is_none(){
            println!("No MX servers found, sorry.");
            break;
        }
        send_combinations(&emails,server.unwrap())
    };
}
