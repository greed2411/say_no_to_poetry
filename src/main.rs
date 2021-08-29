// References:

// https://python-poetry.org/docs/dependency-specification/
// https://python-poetry.org/docs/cli/#export

use std::collections::HashMap;
use std::env;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;
use toml::Value as Toml;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"["\\]"#).unwrap();
}

fn convert_pip_deps_map_to_text(pip_deps: HashMap<String, String>) -> String {
    let mut txt_data = String::new();

    for (key, val) in pip_deps.iter() {
        let formatted_txt = format!("{}{}\n", key, val);
        txt_data.push_str(&formatted_txt)
    }

    txt_data
}

fn process_poetry_version(poetry_version: &String) -> String {
    let mut op_with_version = RE.replace_all(poetry_version, "").to_string();

    if op_with_version.starts_with("^") {
        op_with_version = op_with_version.replace("^", ">=");
    } else {
        op_with_version = "==".to_string() + &op_with_version;
    }

    op_with_version
}

fn processing_deps(poetry_deps: HashMap<String, String>) -> HashMap<String, String> {
    let mut processed_deps = HashMap::new();

    for (key, val) in poetry_deps.iter() {
        let pip_val = process_poetry_version(val);
        processed_deps.insert(key.to_string(), pip_val);
    }

    if processed_deps.contains_key("python") {
        processed_deps.remove("python");
    }

    processed_deps
}

fn merging_deps(all_deps: Vec<&Toml>) -> HashMap<String, String> {
    let mut merged_deps = HashMap::new();

    for dep_toml in all_deps.iter() {
        let dep_hashmap = accumulate_dependencies(dep_toml);
        for (key, val) in dep_hashmap.iter() {
            merged_deps.insert(key.to_string(), val.to_string());
        }
    }

    merged_deps
}

fn get_version(toml_value: &Toml) -> String {
    match toml_value {
        Toml::Table(table) => table
            .get("version")
            .expect("trying to get version")
            .to_string(),
        Toml::String(s) => s.to_string(),
        _ => String::new(),
    }
}

fn accumulate_dependencies(dependencies: &toml::Value) -> HashMap<String, String> {
    let mut dep_with_version = HashMap::new();
    let _table_contents = match dependencies {
        Toml::Table(table) => {
            for (key, val) in table.into_iter() {
                let name_of_dependency = key.clone();
                let version_value = get_version(val);
                dep_with_version.insert(name_of_dependency, version_value);
            }
        }
        _ => {}
    };

    dep_with_version
}

fn validate_toml(parsed_toml: &Toml) -> bool {
    let mut is_it_good_enough: bool = false;

    if parsed_toml.get("tool") != None
        && parsed_toml["tool"].is_table()
        && parsed_toml["tool"].get("poetry") != None
        && parsed_toml["tool"]["poetry"].is_table()
        && ((parsed_toml["tool"]["poetry"].get("dependencies") != None
            && parsed_toml["tool"]["poetry"]["dependencies"].is_table())
            || (parsed_toml["tool"]["poetry"].get("dev-dependencies") != None
                && parsed_toml["tool"]["poetry"]["dev-dependencies"].is_table()))
    {
        is_it_good_enough = true;
    }

    if !is_it_good_enough {
        println!("failed to identify both: \n\t[tool.poetry.dependencies] & \n\t[tool.poetry.dev-dependencies] in pyproject.toml")
    }
    is_it_good_enough
}

fn get_poetry_dependencies(parsed_toml: &Toml) -> Vec<&Toml> {
    let mut pd: Vec<&Toml> = Vec::new();

    if parsed_toml["tool"]["poetry"].get("dependencies") != None {
        pd.push(&parsed_toml["tool"]["poetry"]["dependencies"])
    }

    if parsed_toml["tool"]["poetry"].get("dev-dependencies") != None {
        pd.push(&parsed_toml["tool"]["poetry"]["dev-dependencies"])
    }

    pd
}

fn parse_string_into_toml(raw_text: &str) -> toml::Value {
    toml::from_str(raw_text).unwrap()
}

fn read_file(path_to_pyproject_toml: &String) -> String {
    fs::read_to_string(path_to_pyproject_toml).expect("Unable to read file")
}

fn write_file(output_text: String) {
    fs::write("./requirements.txt", output_text).expect("Failed to write!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_pyproject_toml = &args[1];
    if path_to_pyproject_toml.ends_with(".toml") {
        let parsed_toml = {
            let raw_text = read_file(path_to_pyproject_toml);
            let toml_parsed = parse_string_into_toml(&raw_text);
            toml_parsed
        };
        if validate_toml(&parsed_toml) {
            let all_deps: Vec<&Toml> = get_poetry_dependencies(&parsed_toml);
            let merged_deps = merging_deps(all_deps);
            let pip_processed_deps = processing_deps(merged_deps);
            let requirements_txt_format = convert_pip_deps_map_to_text(pip_processed_deps);
            write_file(requirements_txt_format);
        }
    } else {
        println!("Usage:\n\tsay_no_to_poetry pyproject.toml\n\n\tThis outputs a requirements.txt")
    }
}

#[cfg(test)]
mod tests {

    use super::process_poetry_version;

    #[test]
    fn test_poetry_version_conversion() {
        assert_eq!(process_poetry_version(&"^6.2.4".to_string()), ">=6.2.4");
        assert_eq!(
            process_poetry_version(&"^2021.4.11-beta.34".to_string()),
            ">=2021.4.11-beta.34"
        );
        assert_eq!(process_poetry_version(&"\"^1.2\"".to_string()), ">=1.2");
        assert_eq!(process_poetry_version(&"6.2.4".to_string()), "==6.2.4");
    }
}
