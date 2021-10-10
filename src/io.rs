use std::io;

    pub fn show(text: &str) {
        println!("{}", text);
    }

    pub fn wait() {
        println!("would wait for input here");
    }

    pub fn query(text: &str, choices: Vec<&str>, allow_save: bool) -> u8 {
        if text != "" {
            show(&text);
        }

        let mut c = 1;
        for ch in &choices {
            show(&format!("{}. {}", c, ch));
            c+=1;
        }

        let mut result;
        loop {
            //print!(">");
            result = String::new();
            io::stdin().read_line(&mut result)
                .expect("Failed to read line");
            
            match result.trim() {
                "s" => {
                    if allow_save {
                        show(&String::from("Would save here"));
                    }
                }
                "r" => {
                    if allow_save {
                        show(&String::from("Would restore here"));
                    }
                }
                "q" => {
                    return 0
                }
                _ => (),
            }

            let num_result : u8 = match result.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            if (num_result as usize) < choices.len() {return num_result}

        }
    }

    //TODO: add load_file function