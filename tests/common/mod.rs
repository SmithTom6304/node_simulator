use std::{
    io::{BufRead, BufReader},
    process::{Child, ChildStdin, ChildStdout, Stdio},
    slice::Iter,
    sync::mpsc,
    thread,
    time::Duration,
};

pub struct Binary {}

impl Binary {
    pub fn get() -> std::process::Child {
        test_bin::get_test_bin("node_simulator")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }

    pub fn kill(mut process: Child) {
        process.kill().unwrap();
    }
}

pub struct Write {}

impl Write {
    pub fn write_lines_to_cli(mut std_in: ChildStdin, lines: Iter<&str>) {
        for line in lines {
            let line = format! {"{line}\n"};
            std::io::Write::write_all(&mut std_in, line.as_bytes()).unwrap();
        }
    }

    pub fn write_line_to_cli(std_in: ChildStdin, line: &str) {
        Self::write_lines_to_cli(std_in, vec![line].iter());
    }
}

pub struct Read {}

impl Read {
    pub fn read_from_cli(std_out: ChildStdout) -> String {
        let (tx, rx) = mpsc::channel();
        let mut output_string = String::new();
        let mut std_out_buffer = BufReader::new(std_out);

        thread::spawn(move || {
            while std_out_buffer
                .read_line(&mut output_string)
                .expect("Error reading from child output")
                != 0
            {
                tx.send(output_string.clone()).unwrap();
                output_string.clear();
            }
        });
        thread::sleep(Duration::new(1, 0));
        let mut output = String::new();
        while let Ok(result) = rx.try_recv() {
            output.push_str(&result);
        }
        output
    }
}
