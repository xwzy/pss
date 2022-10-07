pub mod grep {
    pub fn grep_impl(pattern: String, path: std::path::PathBuf) {
        println!(
            "Grep command with pattern: {}, path: {}",
            pattern,
            path.display()
        );
    }
}
