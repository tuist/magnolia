use std::path::PathBuf;

#[test]
fn test_detect_bitrise_config() {
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let configs = magnolia::migrate::detect_source_configs(&fixtures_path).unwrap();

    // Should find Bitrise config
    assert!(configs.iter().any(|c| matches!(
        c.ci_system,
        magnolia::migrate::CISystem::Bitrise
    )));
}

#[test]
fn test_detect_codemagic_config() {
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let configs = magnolia::migrate::detect_source_configs(&fixtures_path).unwrap();

    // Should find Codemagic config
    assert!(configs.iter().any(|c| matches!(
        c.ci_system,
        magnolia::migrate::CISystem::Codemagic
    )));
}

#[test]
fn test_detect_circleci_config() {
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let configs = magnolia::migrate::detect_source_configs(&fixtures_path).unwrap();

    // Should find CircleCI config
    assert!(configs.iter().any(|c| matches!(
        c.ci_system,
        magnolia::migrate::CISystem::CircleCI
    )));
}

#[test]
fn test_detect_all_configs() {
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let configs = magnolia::migrate::detect_source_configs(&fixtures_path).unwrap();

    // Should find all three CI configs in fixtures
    assert_eq!(configs.len(), 3, "Should detect all three CI configs");
}

#[test]
fn test_ci_system_names() {
    assert_eq!(magnolia::migrate::CISystem::Bitrise.name(), "Bitrise");
    assert_eq!(magnolia::migrate::CISystem::Codemagic.name(), "Codemagic");
    assert_eq!(magnolia::migrate::CISystem::CircleCI.name(), "CircleCI");
    assert_eq!(magnolia::migrate::CISystem::GitHubActions.name(), "GitHub Actions");
    assert_eq!(magnolia::migrate::CISystem::GitLabCI.name(), "GitLab CI");
    assert_eq!(magnolia::migrate::CISystem::ForgejoActions.name(), "Forgejo Actions");
}

#[test]
fn test_target_paths() {
    let base = PathBuf::from("/tmp/repo");

    let github_path = magnolia::migrate::CISystem::GitHubActions
        .target_path(&base, Some("test.yml"));
    assert_eq!(github_path, base.join(".github/workflows/test.yml"));

    let gitlab_path = magnolia::migrate::CISystem::GitLabCI
        .target_path(&base, None);
    assert_eq!(gitlab_path, base.join(".gitlab-ci.yml"));

    let forgejo_path = magnolia::migrate::CISystem::ForgejoActions
        .target_path(&base, Some("ci.yml"));
    assert_eq!(forgejo_path, base.join(".forgejo/workflows/ci.yml"));
}
