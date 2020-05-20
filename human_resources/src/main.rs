use std::collections::HashMap;
use std::io;

fn main() {
    let mut ress = HashMap::new();

    loop {
        println!("Command ↓");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read command");

        let (dep, res) = parse_add_command(&cmd);
        add_resource(&mut ress, dep, res);

        println!("Human resources: ");
        println!("{:#?}", ress);
    }
}

fn parse_add_command(cmd: &String) -> (&str, &str) {
    let cmd: Vec<&str> = cmd.trim().split(' ').collect();
    if cmd.len() != 4 {
        return ("", "");
    }

    (cmd[3], cmd[1])
}

fn add_resource(deps: &mut HashMap<String, Vec<String>>, dep: &str, res: &str) {
    let ress = deps.entry(String::from(dep)).or_insert(Vec::new());

    ress.push(String::from(res));
}
