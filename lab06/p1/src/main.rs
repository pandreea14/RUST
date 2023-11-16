use std::fs;
use std::io;
use std::path::Path;

trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &Vec<&str>);
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &'static str {
        "ping"
    }

    fn exec(&mut self, _args: &Vec<&str>) {
        println!("pong");
    }
}

struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> &'static str {
        "count"
    }

    fn exec(&mut self, args: &Vec<&str>) {
        println!("{} args", args.len());
    }
}

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &'static str {
        "times"
    }

    fn exec(&mut self, _args: &Vec<&str>) {
        self.count += 1;
        println!(
            "{} command was executed {} times",
            self.get_name(),
            self.count
        );
    }
}

struct CheckupCommand;

impl Command for CheckupCommand {
    fn get_name(&self) -> &'static str {
        "check"
    }

    fn exec(&mut self, args: &Vec<&str>) {
        if let Some(name) = args.first() {
            println!("Ce faci {}?!?", name);
        } else {
            println!("No one to check up on :(");
        }
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self) -> Result<(), io::Error> {
        let s = fs::read_to_string(Path::new(
            "C:\\Users\\Andreea\\OneDrive\\Desktop\\RUST\\lab06\\p1\\src\\input.txt",
        ))?;

        for line in s.lines() {
            let mut parts = line.split_whitespace();

            while let Some(p) = parts.next() {
                if p == "stop" {
                    println!("Stopped programme");
                    return Ok(());
                }

                let args: Vec<&str> = parts.clone().collect();
                let mut command = None;

                for c in &mut self.commands {
                    if c.get_name() == p {
                        command = Some(c);
                        break;
                    }
                }

                if let Some(command) = command {
                    command.exec(&args);
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CheckupCommand {}));

    if let Err(err) = terminal.run() {
        println!("Error: {}", err);
    }
}