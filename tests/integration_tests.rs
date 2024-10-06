#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_integration_cli_and_handlers() {
        let mut cmd = Command::cargo_bin("LLM-based-intent-recognition").unwrap();
        cmd.write_stdin("test input\n")
            .assert()
            .success()
            .stdout(
                predicates::str::contains("Please enter a string:\n")
                    .and(predicates::str::contains("test input\n")),
            );
    }
}