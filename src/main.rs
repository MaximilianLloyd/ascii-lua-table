use std::io::{self, Read};

fn read_std_in() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn find_longest_line(input: &str) -> &str {
    return input.lines().max_by_key(|line| line.len()).unwrap()
}

fn str_to_lua_table(s: &str) -> String {
    let mut result = String::new();
    let mut first = true;

    let longest_line = find_longest_line(s);
    let longest_line_len = longest_line.len();

    for line in s.lines() {
        if first {
            first = false;
        } else {
            result.push_str(", \n");
        }

        let line_len = line.len();

        let diff = longest_line_len - line_len;

        let to_push: String;

        let newline = line.replace("\\", "\\\\").replace("\"", "\\\"");

        if line_len < longest_line_len {
            let padding = " ".repeat(diff);

            to_push = format!("\"{}{}\"", newline, padding)
        } else {
            to_push = format!("\"{}\"", newline)
        }

        // transform to escaped lua string but keep quotes on the end

        result.push_str(&to_push);
        println!("Diff: {}", diff);
    }

    return format!("{{{}}}", result)
}

fn main() {
    let input = read_std_in();

    match input {
        Ok(s) => {
            let lua_table = str_to_lua_table(&s);
            print!("Lua table:\n{}\n", lua_table);
        },
        Err(e) => println!("Error: {}", e),
    }
}
