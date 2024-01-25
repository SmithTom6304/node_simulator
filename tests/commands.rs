mod common;

const EXPECTED_HELP_COMMAND_OUTPUT: &str = r#"Running node_simulator...
Commands:
  add           
  remove        
  set           
  get           
  toggle-scene  
  close         
  step          
  help          Print this message or the help of the given subcommand(s)

"#;

const EXPECTED_ADD_NODE_COMMAND_OUTPUT: &str = r#"Running node_simulator...
Node 1:
	position: x: 1, y: 2, z: 3
"#;

const EXPECTED_REMOVE_NODE_COMMAND_OUTPUT: &str = r#"Running node_simulator...
Error displaying node information for node with id 1 - no node with that id exists
"#;

//#[test]
fn can_execute_help_command() {
    let mut process = common::Binary::get();
    let std_in = process.stdin.take().expect("Child had no stdin");
    let std_out = process.stdout.take().expect("Child had no stdout");

    common::Write::write_line_to_cli(std_in, "--help");
    let output = common::Read::read_from_cli(std_out);

    assert_eq!(EXPECTED_HELP_COMMAND_OUTPUT, output);

    common::Binary::kill(process);
}

//#[test]
fn can_execute_add_node_command() {
    let mut process = common::Binary::get();
    let std_in = process.stdin.take().expect("Child had no stdin");
    let std_out = process.stdout.take().expect("Child had no stdout");

    let commands = vec![
        "add node --id 1 --position 1,2,3",
        "get node --id 1 --position",
    ];

    common::Write::write_lines_to_cli(std_in, commands.iter());
    let output = common::Read::read_from_cli(std_out);

    assert_eq!(EXPECTED_ADD_NODE_COMMAND_OUTPUT, output);

    common::Binary::kill(process);
}

//#[test]
fn can_execute_remove_node_command() {
    let mut process = common::Binary::get();
    let std_in = process.stdin.take().expect("Child had no stdin");
    let std_out = process.stdout.take().expect("Child had no stdout");

    let commands = vec![
        "add node --id 1 --position 1,2,3",
        "remove node --id 1",
        "get node --id 1 --position",
    ];

    common::Write::write_lines_to_cli(std_in, commands.iter());
    let output = common::Read::read_from_cli(std_out);

    assert_eq!(EXPECTED_REMOVE_NODE_COMMAND_OUTPUT, output);

    common::Binary::kill(process);
}
