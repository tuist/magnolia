use std::path::PathBuf;
use std::process::Command;

fn get_binary_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Assuming the test is run via 'cargo test', the binary should be in target/debug
    // Adjust path logic to find the binary relative to the package root
    manifest_dir.join("target/debug/magnolia")
}

fn get_fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

fn has_agent_cli() -> bool {
    Command::new("claude")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        || Command::new("codex")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
}

#[test]
fn test_help_documentation() {
    let bin = get_binary_path();
    let output = Command::new(bin)
        .arg("--help")
        .output()
        .expect("Failed to execute binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--non-interactive"),
        "Help should document --non-interactive"
    );
    assert!(stdout.contains("--job"), "Help should document --job");
}

#[test]
fn test_migrate_non_interactive_ambiguous_fail() {
    let bin = get_binary_path();
    let output = Command::new(bin)
        .args(["migrate", "--non-interactive", "--dry-run"])
        .arg("--path")
        .arg(get_fixtures_path())
        .output()
        .expect("Failed to execute binary");

    assert!(
        !output.status.success(),
        "Should fail when multiple sources exist"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Note: The CLI might print the error to stdout or stderr depending on how anyhow is handled
    // Checking both just in case, though anyhow usually goes to stderr
    let output_combined = format!("{}{}", String::from_utf8_lossy(&output.stdout), stderr);

    assert!(
        output_combined.contains("Multiple source CI configurations found"),
        "Should report ambiguous sources error"
    );
}

#[test]
fn test_migrate_non_interactive_explicit_success() {
    // Skip test if no agent CLI is available (e.g., in CI)
    if !has_agent_cli() {
        eprintln!("Skipping test: no agent CLI (claude/codex) available");
        return;
    }

    let bin = get_binary_path();
    let output = Command::new(bin)
        .args(["migrate", "Bitrise", "--non-interactive", "--dry-run"])
        .arg("--path")
        .arg(get_fixtures_path())
        .output()
        .expect("Failed to execute binary");

    if !output.status.success() {
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
    assert!(
        output.status.success(),
        "Should succeed with explicit source"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Migration complete"),
        "Should indicate completion"
    );
}

#[test]
fn test_run_pipeline_non_interactive_ambiguous_fail() {
    let bin = get_binary_path();
    // Running in fixtures dir where multiple pipelines exist
    let fixtures = get_fixtures_path();

    let output = Command::new(bin)
        .current_dir(&fixtures)
        .arg("--non-interactive")
        .output()
        .expect("Failed to execute binary");

    assert!(
        !output.status.success(),
        "Should fail when multiple pipelines exist"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let output_combined = format!("{}{}", String::from_utf8_lossy(&output.stdout), stderr);

    assert!(
        output_combined.contains("Multiple pipelines found"),
        "Should report ambiguous pipelines error"
    );
}

#[test]
fn test_run_pipeline_non_interactive_explicit_success() {
    let bin = get_binary_path();

    // Create a temp dir to hold a standard-named .gitlab-ci.yml
    let temp_dir = std::env::temp_dir().join("magnolia_test_explicit_success");
    std::fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let pipeline_path = temp_dir.join(".gitlab-ci.yml");

    let content = r#"
stages:
  - test
echo_job:
  stage: test
  script:
    - echo "Non-interactive test success"
"#;
    std::fs::write(&pipeline_path, content).expect("Failed to write pipeline file");

    let output = Command::new(bin)
        .arg(&pipeline_path)
        .arg("--job")
        .arg("echo_job")
        .arg("--non-interactive")
        .output()
        .expect("Failed to execute binary");

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);

    if !output.status.success() {
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Running job"),
        "Should attempt to run the job"
    );
    assert!(
        stdout.contains("echo_job"),
        "Should mention the correct job"
    );
}

#[test]
fn test_run_pipeline_non_interactive_missing_job_fail() {
    let bin = get_binary_path();
    // Use the actual fixture file which we know has multiple jobs
    let pipeline = get_fixtures_path().join(".gitlab-ci.yml");

    let output = Command::new(bin)
        .arg(pipeline)
        .arg("--non-interactive")
        .output()
        .expect("Failed to execute binary");

    assert!(
        !output.status.success(),
        "Should fail when job is unspecified and multiple exist"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let output_combined = format!("{}{}", String::from_utf8_lossy(&output.stdout), stderr);

    if !output_combined.contains("Multiple jobs found") {
        println!("Actual Output:\n{}", output_combined);
    }

    assert!(
        output_combined.contains("Multiple jobs found"),
        "Should report ambiguous jobs error"
    );
}
