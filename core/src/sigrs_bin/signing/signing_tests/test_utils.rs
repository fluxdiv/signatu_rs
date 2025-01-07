use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// generate new testing config file from config type
pub fn gen_test_config(
    config_type: ConfigFileType,
    output_path: &Path,
) -> Result<File, std::io::Error> {
    // get template content
    let content = config_content(config_type);
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(output_path)?;
    file.write_all(content.as_bytes())?;
    Ok(file)
}

/// clean up
pub fn delete_test_file(output_path: &Path) -> Result<(), std::io::Error> {
    if output_path.exists() {
        fs::remove_file(output_path)?;
    }
    Ok(())
}

pub enum ConfigFileType {
    // Rust
    CargoToml(ConfigTemplate),

    // jsts
    PackageJson(ConfigTemplate),

    // python
    PyProjectToml(ConfigTemplate),
    // SetupCfg(ConfigTemplate),

    // php
    ComposerJson(ConfigTemplate),

    // ruby
    // GemSpec(ConfigTemplate),
}

pub enum ConfigTemplate {
    NoAuthors,
    EmptyAuthors,
    WithAuthors,
    /// Misc unique variants for each type that don't match others
    /// can pass config file content directly && use within if needed or just match
    /// on variants for no clutter
    Unique(String),
}


/// get content for a specific configuration file template
fn config_content(config_type: ConfigFileType) -> String {
    match config_type {
        ConfigFileType::CargoToml(template) => cargo_template(template),
        ConfigFileType::PackageJson(template) => packagejson_template(template),
        ConfigFileType::PyProjectToml(template) => pyprojecttoml_template(template),
        ConfigFileType::ComposerJson(template) => composerjson_template(template),
        // ConfigFileType::SetupCfg(template) => setupcfg_template(template),
        // ConfigFileType::GemSpec(template) => gemspec_template(template),
        // ConfigFileType::PomXml(template) => pomxml_template(template), 
    }
}


fn cargo_template(template: ConfigTemplate) -> String {
    match template {
        ConfigTemplate::NoAuthors => r#"
[package]
name = "example_project"
version = "0.1.0"
description = "An example Rust project"
            "#.to_string(),

        ConfigTemplate::EmptyAuthors => r#"
[package]
name = "example_project"
version = "0.1.0"
description = "An example Rust project"
authors = []
            "#.to_string(),

        ConfigTemplate::WithAuthors | _ => r#"
[package]
name = "example_project"
version = "0.1.0"
description = "An example Rust project"
authors = ["Jane Doe <jane.doe@example.com>"]
            "#.to_string()
    }
}

fn packagejson_template(template: ConfigTemplate) -> String {
    match template {
        ConfigTemplate::NoAuthors => r#"
{
  "name": "example-project",
  "version": "0.1.0"
}
            "#.to_string(),

        ConfigTemplate::EmptyAuthors => r#"
{
  "name": "example-project",
  "version": "0.1.0",
  "contributors": []
}
            "#.to_string(),

        ConfigTemplate::WithAuthors | _ => r#"
{
  "name": "example-project",
  "version": "0.1.0",
  "contributors": [
    {
      "name": "Barney Rubble",
      "email": "b@rubble.com",
      "url": "http://barnyrubble.tumblr.com/"
    },
    {
      "author": "Barney Rubble <b@rubble.com> (http://barnyrubble.tumblr.com/)"
    },
    "Jane Doe <jane.doe@example.com>"
  ]
}
            "#.to_string()
    }
}

fn pyprojecttoml_template(template: ConfigTemplate) -> String {
    match template {
        ConfigTemplate::NoAuthors => r#"
[tool.poetry]
name = "example-project"
version = "0.1.0"
description = "An example Python project"
            "#.to_string(),

        ConfigTemplate::EmptyAuthors => r#"
[tool.poetry]
name = "example-project"
version = "0.1.0"
description = "An example Python project"
authors = []
            "#.to_string(),

        ConfigTemplate::WithAuthors | _ => r#"
[tool.poetry]
name = "example-project"
version = "0.1.0"
description = "An example Python project"
authors = ["Jane Doe <jane.doe@example.com>"]
            "#.to_string()
    }
}

fn _setupcfg_template(template: ConfigTemplate) -> String {
    match template {
        ConfigTemplate::NoAuthors => r#"
[metadata]
name = example_project
version = 0.1.0
description = Example project description.

[options]
install_requires =
    numpy
    requests
python_requires = >=3.6
            "#.to_string(),

        ConfigTemplate::EmptyAuthors => r#"
[metadata]
name = example_project
version = 0.1.0
author =
author_email =
description = Example project description.

[options]
install_requires =
    numpy
    requests
python_requires = >=3.6
            "#.to_string(),

        ConfigTemplate::WithAuthors => r#"
[metadata]
name = example_project
version = 0.1.0
author = Jane Doe, John Smith
author_email = jane.doe@example.com, john@example.com
description = Example project description.

[options]
install_requires =
    numpy
    requests
python_requires = >=3.6
            "#.to_string(),
        // Only unique for setup.cfg is no metadata field
        ConfigTemplate::Unique(_) => r#"
[options]
install_requires =
    numpy
    requests
python_requires = >=3.6
            "#.to_string()
    }
}


fn composerjson_template(template: ConfigTemplate) -> String {
    match template {
        ConfigTemplate::NoAuthors => r#"
{
  "name": "example-project",
  "description": "An example PHP project",
  "version": "1.0.0",
  "require": {
    "php": ">=7.4",
    "monolog/monolog": "^2.0"
  }
}
            "#.to_string(),

        ConfigTemplate::EmptyAuthors => r#"
{
  "name": "example-project",
  "description": "An example PHP project",
  "version": "1.0.0",
  "authors": [],
  "require": {
    "php": ">=7.4",
    "monolog/monolog": "^2.0"
  }
}
            "#.to_string(),

        ConfigTemplate::WithAuthors | _ => r#"
{
  "name": "example-project",
  "description": "An example PHP project",
  "version": "1.0.0",
  "authors": [
    {
      "name": "Jane Doe",
      "email": "jane.doe@example.com",
      "role": "Developer"
    },
    {
      "name": "John Smith",
      "email": "john.smith@example.com",
      "role": "Maintainer"
    }
  ],
  "require": {
    "php": ">=7.4",
    "monolog/monolog": "^2.0"
  }
}
            "#.to_string()
    }
}

fn _gemspec_template(template: ConfigTemplate) -> String {
    match template {
        ConfigTemplate::NoAuthors => r#"
Gem::Specification.new do |spec|
  spec.name          = "example_project"
  spec.version       = "0.1.0"
  spec.summary       = "An example Ruby gem"
  spec.description   = "This gem provides an example of how to use Ruby gems."
  spec.files         = Dir["lib/**/*.rb"]
  spec.homepage      = "https://example.com"
  spec.license       = "MIT"
end
        "#.to_string(),

        ConfigTemplate::EmptyAuthors => r#"
Gem::Specification.new do |spec|
  spec.name          = "example_project"
  spec.version       = "0.1.0"
  spec.summary       = "An example Ruby gem"
  spec.description   = "This gem provides an example of how to use Ruby gems."
  spec.authors       = []
  spec.email         = []
  spec.files         = Dir["lib/**/*.rb"]
  spec.homepage      = "https://example.com"
  spec.license       = "MIT"
end
        "#.to_string(),

        ConfigTemplate::WithAuthors | _ => r#"
Gem::Specification.new do |spec|
  spec.name          = "example_project"
  spec.version       = "0.1.0"
  spec.summary       = "An example Ruby gem"
  spec.description   = "This gem provides an example of how to use Ruby gems."
  spec.authors       = ["Jane Doe", "John Smith"]
  spec.email         = ["jane.doe@example.com", "john.smith@example.com"]
  spec.files         = Dir["lib/**/*.rb"]
  spec.homepage      = "https://example.com"
  spec.license       = "MIT"
end
        "#.to_string()
    }
}


fn _template(template: ConfigTemplate) -> String {
    match template {
        ConfigTemplate::NoAuthors => r#""#.to_string(),

        ConfigTemplate::EmptyAuthors => r#""#.to_string(),

        ConfigTemplate::WithAuthors | _ => r#""#.to_string()
    }
}


// fn pomxml_template(template: ConfigTemplate) -> String {
//         match template {
//             ConfigTemplate::NoAuthors => r#"
// <project>
//   <modelVersion>4.0.0</modelVersion>
//   <groupId>com.example</groupId>
//   <artifactId>example-project</artifactId>
//   <version>1.0.0</version>
// </project>
//             "#.to_string(),
//
//             ConfigTemplate::EmptyAuthors => r#"
// <project>
//   <modelVersion>4.0.0</modelVersion>
//   <groupId>com.example</groupId>
//   <artifactId>example-project</artifactId>
//   <version>1.0.0</version>
//   <developers>
//   </developers>
//   <contributors>
//   </contributors>
// </project>
//             "#.to_string(),
//
//             ConfigTemplate::WithAuthors | _ => r#"
// <project>
//   <modelVersion>4.0.0</modelVersion>
//   <groupId>com.example</groupId>
//   <artifactId>example-project</artifactId>
//   <version>1.0.0</version>
//   <developers>
//     <developer>
//       <id>jdoe</id>
//       <name>Jane Doe</name>
//       <email>jane.doe@example.com</email>
//     </developer>
//   </developers>
//   <contributors>
//     <contributor>
//       <name>John Smith</name>
//       <email>john.smith@example.com</email>
//     </contributor>
//   </contributors>
// </project>
//             "#.to_string()
//         }
// }
