use std::io::Write;

pub mod grep {
    use std::io::Write;

    pub fn grep_impl(pattern: String, path: std::path::PathBuf) {
        println!(
            "Grep command with pattern: {}, path: {}",
            pattern,
            path.display()
        );

        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };
        let pb = indicatif::ProgressBar::new(content.lines().count() as u64);
        let mut i = 0;
        let mut string_builder = content
            .lines()
            .filter(|line| {
                std::thread::sleep(std::time::Duration::from_millis(20));
                i += 1;
                pb.set_position(i as u64);
                line.contains(&pattern)
            })
            .collect::<Vec<&str>>()
            .join("\n");
        string_builder.push_str("\n");
        pb.finish();
        let stdout = std::io::stdout();
        let mut handle = std::io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
        handle.write_all(string_builder.as_bytes()).unwrap();
    }
}

#[test]
fn test_grep() {
    let path = std::path::PathBuf::from("src/subcmd/grep.rs");
    grep::grep_impl("let".to_string(), path);
}
