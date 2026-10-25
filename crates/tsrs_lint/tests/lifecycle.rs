use std::io::{self, Write};

use tsrs_core::json::{self, Value};
use tsrs_lint::engine::{self, Options};
use tsrs_lint::payload::{Config, ConfiguredRule, Payload};

struct BrokenWriter;

impl Write for BrokenWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("injected output failure"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn repeated_library_calls_release_request_state_on_success_and_failure() {
    std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(|| {
            let cwd = std::env::current_dir().unwrap().to_string_lossy().into_owned();
            let files = (0..16)
                .map(|index| format!("{cwd}/target/lint-lifecycle/virtual{index}.ts"))
                .collect::<Vec<_>>();
            let mut payload = Payload {
                configs: vec![Config {
                    file_paths: files.clone(),
                    rules: vec![ConfiguredRule {
                        name: "await-thenable".into(),
                        ..Default::default()
                    }],
                }],
                source_overrides: files
                    .into_iter()
                    .map(|file| (file, "export {}; await 123; const wrong: string = 1;".into()))
                    .collect(),
                report_semantic: false,
                ..Default::default()
            };
            for semantic in [false, true] {
                payload.report_semantic = semantic;
                for _ in 0..3 {
                    assert!(
                        engine::run(&payload, Options::default(), &cwd, &mut BrokenWriter).is_err()
                    );
                    let mut output = Vec::new();
                    engine::run(&payload, Options::default(), &cwd, &mut output).unwrap();
                    let mut count = 0;
                    let mut bytes = output.as_slice();
                    while !bytes.is_empty() {
                        let length = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
                        assert_eq!(bytes[4], 1);
                        bytes = &bytes[length + 5..];
                        count += 1;
                    }
                    assert_eq!(count, if semantic { 32 } else { 16 });
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn one_request_lints_each_project_after_freeing_the_previous_one() {
    std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(|| {
            let cwd = std::env::current_dir().unwrap().to_string_lossy().into_owned();
            let base = format!("{cwd}/target/lint-project-regions-lifecycle");
            let mut configs = Vec::new();
            let mut source_overrides = Vec::new();
            let mut files = Vec::new();
            // One-file projects lint serially on the program's checker, four-file ones on workers; every project
            // parses, binds and checks its own lib files after the previous project's regions were freed.
            for project in 0..6 {
                let directory = format!("{base}/p{project}");
                let names = (0..if project % 2 == 0 { 1 } else { 4 })
                    .map(|index| format!("f{index}.ts"))
                    .collect::<Vec<_>>();
                source_overrides.push((
                    format!("{directory}/tsconfig.json"),
                    format!(
                        r#"{{"compilerOptions":{{"strict":true,"target":"es2022","module":"esnext","types":[]}},"files":{names:?}}}"#
                    ),
                ));
                let paths = names.iter().map(|name| format!("{directory}/{name}")).collect::<Vec<_>>();
                for path in &paths {
                    source_overrides.push((
                        path.clone(),
                        "export {}; await 123; const wrong: string = 1; Promise.resolve(1);".into(),
                    ));
                }
                files.extend(paths.iter().cloned());
                configs.push(Config {
                    file_paths: paths,
                    rules: ["await-thenable", "no-floating-promises"]
                        .map(|name| ConfiguredRule { name: name.into(), ..Default::default() })
                        .into(),
                });
            }
            let payload = Payload {
                configs,
                source_overrides: source_overrides.into_iter().collect(),
                report_semantic: true,
                ..Default::default()
            };
            let mut output = Vec::new();
            engine::run(&payload, Options::default(), &cwd, &mut output).unwrap();
            let mut found = std::collections::BTreeMap::<String, Vec<String>>::new();
            let mut bytes = output.as_slice();
            while !bytes.is_empty() {
                let length = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
                assert_eq!(bytes[4], 1);
                let Value::Object(diagnostic) =
                    json::unmarshal(std::str::from_utf8(&bytes[5..length + 5]).unwrap()).unwrap()
                else {
                    panic!()
                };
                let Value::String(file) = &diagnostic["file_path"] else { panic!() };
                let name = match (diagnostic.get("rule"), &diagnostic["message"]) {
                    (Some(Value::String(rule)), _) => rule.clone(),
                    (None, Value::Object(message)) => {
                        let Value::String(id) = &message["id"] else { panic!() };
                        id.clone()
                    }
                    _ => panic!(),
                };
                found.entry(file.clone()).or_default().push(name);
                bytes = &bytes[length + 5..];
            }
            assert_eq!(found.len(), files.len());
            for file in &files {
                let mut names = found.remove(file).unwrap_or_default();
                names.sort();
                assert_eq!(
                    names,
                    ["TS2322", "await-thenable", "no-floating-promises"],
                    "{file}"
                );
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn repeated_library_calls_observe_changed_project_configuration() {
    std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(|| {
            let cwd = std::env::current_dir().unwrap().to_string_lossy().into_owned();
            let base = format!("{cwd}/target/lint-config-cache-lifecycle");
            let file = format!("{base}/virtual.ts");
            let config = format!("{base}/tsconfig.json");
            let mut payload = Payload {
                configs: vec![Config {
                    file_paths: vec![file.clone()],
                    rules: Vec::new(),
                }],
                source_overrides: std::iter::once((file, "export {}; const value: number = null;".into()))
                    .collect(),
                report_semantic: true,
                ..Default::default()
            };
            for strict in [true, false, true, false] {
                payload.source_overrides.insert(
                    config.clone(),
                    format!(
                        r#"{{"compilerOptions":{{"strictNullChecks":{strict},"types":[]}},"files":["virtual.ts"]}}"#
                    ),
                );
                let mut output = Vec::new();
                engine::run(&payload, Options::default(), &cwd, &mut output).unwrap();
                if strict {
                    let length = u32::from_le_bytes(output[..4].try_into().unwrap()) as usize;
                    assert_eq!(output.len(), length + 5);
                    assert_eq!(output[4], 1);
                    assert!(std::str::from_utf8(&output[5..]).unwrap().contains("TS2322"));
                } else {
                    assert_eq!(output, [] as [u8; 0]);
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
