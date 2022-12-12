use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    process::Command,
    time::Instant,
};

use regex::Regex;

fn prompt(prompt: &str) -> String {
    let mut s: String = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn package_json_init(package: String, author: String) {
    let package_regex: Regex =
        Regex::new(r"^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$").unwrap();
    if !package_regex.is_match(&package) {
        panic!(
            r#"Package Name does not match the pattern of "^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$".
        tl;dr: Package can't have capitals, underscores, or dashes"#
        );
    }

    let version: String = "1.0.0".to_owned();

    println!("Initializing package.json... (./{}/package.json)", package);

    let package_json: String = format!(
        include_str!("../template/package.json"),
        package, version, "MIT", author
    );

    let mut package_json_file: File = File::create(format!("./{}/package.json", package)).unwrap();

    package_json_file
        .write_all(package_json.as_bytes())
        .unwrap();
    println!("package.json file created");
}

fn app_ts_init(package: String) {
    println!("Initializing app... (./{}/src/app.ts)", package);
    // mkdir ./package/src/
    fs::create_dir_all(format!("./{}/src", package)).unwrap();

    let mut app_ts_file: File = File::create(format!("./{}/src/app.ts", package)).unwrap();
    let app_ts = include_str!("../template/src/app.ts");
    app_ts_file.write_all(app_ts.as_bytes()).unwrap();
    println!("app.ts file created");
}

fn logger_init(package: String) {
    println!("Initializing logger... (./{}/src/utils/logger.ts)", package);
    // mkdir ./package/src/utils
    fs::create_dir_all(format!("./{}/src/utils", package)).unwrap();
    let mut logger_file: File = File::create(format!("./{}/src/utils/logger.ts", package)).unwrap();
    let logger_ts = include_str!("../template/src/utils/logger.ts");
    logger_file.write_all(logger_ts.as_bytes()).unwrap();
    println!("logger.ts file created");
}

fn tsconfig_json_init(package: String) {
    println!("Initializing tsconfig... (./{}/tsconfig.json)", package);
    let mut tsconfig_json_file = File::create(format!("./{}/tsconfig.json", package)).unwrap();
    let tsconfig_json = include_str!("../template/tsconfig.json");
    tsconfig_json_file
        .write_all(tsconfig_json.as_bytes())
        .unwrap();

    println!("tsconfig.json created");
}

fn nodemon_json_init(package: String) {
    println!(
        "Initializing nodemon config... (./{}/nodemon.json)",
        package
    );

    let nodemon_json = include_str!("../template/nodemon.json");
    let mut nodemon_json_file = File::create(format!("./{}/nodemon.json", package)).unwrap();
    nodemon_json_file
        .write_all(nodemon_json.as_bytes())
        .unwrap();
    println!("nodemon.json created");
}

fn license_generator_init(package: String, author: String) {
    println!("Initializing LICENSE... (./{}/LICENSE)", package);
    use chrono::prelude::*;
    let current_date = chrono::Local::now();
    let current_year: String = current_date.year().to_string();

    let mit_license =
        format!(include_str!("../template/LICENSE"), current_year, author).to_string();
    let mut license_file = File::create(format!("./{}/LICENSE", package)).unwrap();
    license_file.write_all(mit_license.as_bytes()).unwrap();
    println!("LICENSE created");
}

fn readme_init(package: String) {
    println!("Initializing README... (./{}/README.md)", package);
    let readme = include_str!("../template/README.md");
    let mut readme_file = File::create(format!("./{}/README.md", package)).unwrap();
    readme_file.write_all(readme.as_bytes()).unwrap();
    println!("README created");
}

fn env_init(package: String) {
    println!("Initializing .env... (./{}/.env)", package);
    let env = r#""#.to_string();
    let mut env_file = File::create(format!("./{}/.env", package)).unwrap();
    env_file.write_all(env.as_bytes()).unwrap();
    println!(".env created");
}

fn gigignore_init(package: String) {
    println!("Initializing .gitignore... (./{}/.gitignore)", package);
    let gitignore = include_str!("../template/.gitignore");
    let mut gitignore_file = File::create(format!("./{}/.gitignore", package)).unwrap();
    gitignore_file.write_all(gitignore.as_bytes()).unwrap();
    println!(".gitignore created");
}

fn main() {
    // start timer
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut yes = false;
    // see if there is a -y flag
    for arg in args.iter() {
        if arg == "-y" {
            yes = true;
        }
    }

    let mut noauthor = false;
    for arg in args.iter() {
        if arg == "-a" {
            noauthor = true;
        }
    }

    // check for the name of the package, every arg except for -y
    let mut package = String::new();
    for arg in args.iter() {
        if arg != "-y" && arg != "-a" {
            package = arg.to_string();
        }
    }

    println!("This applciation will help you to create a new TypeScript project.");
    println!("It also generates an MIT license file and a README.md file.");
    println!("");
    if !yes {
        let agree = prompt("Are you sure you want to continue? (y/n) ").to_lowercase();
        // regex to match y or yes
        let agree_regex = Regex::new(r"^(y|yes)$").unwrap();
        if !agree_regex.is_match(&agree) && agree != "" {
            println!("Exiting...");
            return;
        }
    }

    if package.len() == 0 && package != "." {
        package = prompt("Package name: ");
    }

    let mut author = "alexng353".to_owned();
    if !noauthor {
        author = prompt("Author: ");
        if author.len() == 0 {
            author = "alexng353".to_owned();
        }
    }

    fs::create_dir_all(format!("./{}", package)).unwrap();
    println!();

    package_json_init(package.clone(), author.clone());
    println!();

    app_ts_init(package.clone());
    println!();

    logger_init(package.clone());
    println!();

    tsconfig_json_init(package.clone());
    println!();

    nodemon_json_init(package.clone());
    println!();

    license_generator_init(package.clone(), author.clone());
    println!();

    readme_init(package.clone());
    println!();

    env_init(package.clone());
    println!();

    gigignore_init(package.clone());
    println!();

    let dir = env::current_dir().unwrap();
    println!("Installing dependencies...");

    if cfg!(target_os = "windows") {
        let mut child = Command::new("cmd")
            .arg("/c")
            .arg("yarn")
            .current_dir(&dir.join(&package))
            .spawn()
            .unwrap();
        child.wait().unwrap();
    } else {
        let mut child = Command::new("yarn")
            .current_dir(&dir.join(&package))
            .spawn()
            .unwrap();
        child.wait().unwrap();
    };

    println!(
        "{}",
        format!("\x1b[32m{}\x1b[0m", "\n\nDependencies installed\n\n")
    );
    // run git init
    println!("Initializing git repository...");

    if cfg!(target_os = "windows") {
        let mut child = Command::new("cmd")
            .arg("/c")
            .arg("git init")
            .current_dir(&dir.join(&package))
            .spawn()
            .unwrap();
        child.wait().unwrap();
    } else {
        let mut child = Command::new("git")
            .arg("init")
            .current_dir(&dir.join(&package))
            .spawn()
            .unwrap();
        child.wait().unwrap();
    };
    println!(
        "{}",
        format!("\x1b[32m{}\x1b[0m", "\n\nGit repository initialized\n\n")
    );

    println!("Package {} created successfully!", package);
    // red print that says "You need to install nodemon if you don't have it"
    // blue print that says "npm i -g nodemon"
    let blue_text: String = "\x1b[34m".to_string();
    let red_text: String = "\x1b[31m".to_string();
    let end_code: String = "\x1b[0m".to_string();
    println!();
    // println!("You need to install nodemon if you don't have it");
    println!(
        "{}You need to install nodemon if you don't have it{}",
        blue_text, end_code
    );
    println!("{}yarn global add nodemon{}", red_text, end_code);
    // end timer, print time elapsed
    let duration = start.elapsed();
    println!();
    println!(
        "Time elapsed:{} {}.{:03} seconds{}",
        "\x1b[32m",
        duration.as_secs(),
        duration.subsec_millis(),
        end_code
    );
}
