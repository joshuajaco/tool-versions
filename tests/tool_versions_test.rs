use std::path::Path;
use tool_versions::ToolVersions;

#[test]
fn it_gets_versions() {
    let path = Path::new("tests/__fixtures__/_tool-versions");
    let tools = ToolVersions::from_file(path).unwrap();

    assert_eq!(
        tools.versions("nodejs"),
        Some(vec!["18.12".to_string(), "system".to_string()])
    );

    assert_eq!(
        tools.versions("ruby"),
        Some(vec!["12".to_string(), "19".to_string()])
    );

    assert_eq!(tools.versions("rust"), Some(vec!["4".to_string()]));

    assert_eq!(
        tools.versions("lua"),
        Some(vec!["19".to_string(), "20".to_string()])
    );
}

#[test]
fn it_sets_versions() {
    let path = Path::new("tests/__fixtures__/_tool-versions");
    let mut tools = ToolVersions::from_file(path).unwrap();

    assert_eq!(
        tools.versions("nodejs"),
        Some(vec!["18.12".to_string(), "system".to_string()])
    );

    tools.set_versions("nodejs", vec!["8", "9", "10"]);

    assert_eq!(
        tools.versions("nodejs"),
        Some(vec!["8".to_string(), "9".to_string(), "10".to_string()])
    );
}

#[test]
fn it_removes_versions() {
    let path = Path::new("tests/__fixtures__/_tool-versions");
    let mut tools = ToolVersions::from_file(path).unwrap();

    assert_eq!(
        tools.versions("nodejs"),
        Some(vec!["18.12".to_string(), "system".to_string()])
    );

    tools.set_versions("nodejs", vec![]);

    assert_eq!(tools.versions("nodejs"), None);
}
