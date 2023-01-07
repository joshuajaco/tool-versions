use std::{env, fs, path::Path};
use tool_versions::{ast, ToolVersions};

#[test]
fn it_works_with_file() {
    let mut tools =
        ToolVersions::from_file(Path::new("tests/__fixtures__/_tool-versions")).unwrap();

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

    assert_eq!(
        tools.errors(),
        vec![
            &ast::SyntaxError::DuplicateIdentifier("nodejs".to_string()),
            &ast::SyntaxError::UnexpectedToken {
                token: 'i',
                expected: "EOL,Comment",
            },
            &ast::SyntaxError::UnexpectedToken {
                token: '#',
                expected: "Version",
            },
            &ast::SyntaxError::UnexpectedEOL {
                expected: "Version",
            },
            &ast::SyntaxError::DuplicateIdentifier("lua".to_string()),
            &ast::SyntaxError::UnexpectedEOL {
                expected: "Version"
            }
        ]
    );

    tools.set_versions("nodejs", vec!["8", "9", "10"]);
    tools.set_versions("ruby", vec!["13"]);
    tools.set_versions("lua", vec![]);

    assert_eq!(
        tools.versions("nodejs"),
        Some(vec!["8".to_string(), "9".to_string(), "10".to_string()])
    );

    assert_eq!(tools.versions("ruby"), Some(vec!["13".to_string()]));

    assert_eq!(tools.versions("rust"), Some(vec!["4".to_string()]));

    assert_eq!(tools.versions("lua"), None);

    let path = env::temp_dir().join("_tool-versions");

    tools.write_file(path.clone()).unwrap();

    let result = fs::read_to_string(path).unwrap();

    assert_eq!(result, "nodejs  8    9 10  # foobar  \nruby    13\n   ## foo ## bar \nrust 4      \n         \nnodejs      12   \n ignored \n# asda\nrust# comment\nrust\nlua   \ngolang \n");
}

#[test]
fn it_works_with_string() {
    let mut tools = ToolVersions::from("nodejs  18.12    system  # foobar  ");

    assert_eq!(
        tools.versions("nodejs"),
        Some(vec!["18.12".to_string(), "system".to_string()])
    );

    tools.set_versions("nodejs", vec!["8", "9", "10"]);

    assert_eq!(
        tools.versions("nodejs"),
        Some(vec!["8".to_string(), "9".to_string(), "10".to_string()])
    );

    let result = tools.write();

    assert_eq!(result, "nodejs  8    9 10  # foobar  \n");
}
