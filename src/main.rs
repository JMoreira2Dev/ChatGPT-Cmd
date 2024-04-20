use chat_gpt_rs::prelude::{Request, Message, Model, Api, Token};
use std::io::{stdin, stdout, Write};
use std::env::args;
use std::process::Command;
use colored::Colorize;

fn logo() -> &'static str {

    r#"

    ____     __                __    ____    ____    ______          ____              ____      
    /\  _`\  /\ \              /\ \__/\  _`\ /\  _`\ /\__  _\        /\  _`\    /'\_/`\/\  _`\    
    \ \ \/\_\\ \ \___      __  \ \ ,_\ \ \L\_\ \ \L\ \/_/\ \/        \ \ \/\_\ /\      \ \ \/\ \  
     \ \ \/_/_\ \  _ `\  /'__`\ \ \ \/\ \ \L_L\ \ ,__/  \ \ \  _______\ \ \/_/_\ \ \__\ \ \ \ \ \ 
      \ \ \L\ \\ \ \ \ \/\ \L\.\_\ \ \_\ \ \/, \ \ \/    \ \ \/\______\\ \ \L\ \\ \ \_/\ \ \ \_\ \
       \ \____/ \ \_\ \_\ \__/.\_\\ \__\\ \____/\ \_\     \ \_\/______/ \ \____/ \ \_\\ \_\ \____/
        \/___/   \/_/\/_/\/__/\/_/ \/__/ \/___/  \/_/      \/_/          \/___/   \/_/ \/_/\/___/ 
                                                                                                                                                                                      
                    ->    [ Interactive Bash environment with the OpenAI API ]

    "#
}

async fn cmd(token_string: &str) -> &'static str {
    
    loop {
        print!("{} ", "gptcmd@OpenAI-API:/$".truecolor(12, 254, 47));
        
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();

        let message = s.trim().to_string();

        if message == "exit" {
            break;
        } 
        else if message == "clear" {
            let _ = Command::new("clear").status();
        }
        else if message.is_empty() {
            continue;
        } else {
            send_request(token_string, message).await;
        }
    };

    "\nExecution finished!\n"
}

async fn send_request(api_token: &str, user_message: String) {

    let token = Token::new(api_token); 
    let api = Api::new(token);

    let request = Request {
    
        model: Model::Gpt35Turbo,
        messages: vec![Message {
            role: "user".to_string(),
            content: user_message,
        }],
        ..Default::default()
    };

    let response = api.chat(request).await;

    if let Ok(response) = response {
        let content = response.choices[0].message.content.replace("\n", "\n");
        println!("\n{} {}\n", "==> ".red(), content);
    }
    else {
        println!("\n{:?}\n", response.err());
    }
} 

#[tokio::main]
async fn main() {

    let args: Vec<String> = args().collect();

    if args.len() <= 1 {

        println!("\nUsage: ./app <OpenAI API Token>\n");
        return;

    } else {

        let token = &args[1];

        println!("{}", logo().bright_blue().bold());
        println!("{}", cmd(token).await);
        return;
    }
}