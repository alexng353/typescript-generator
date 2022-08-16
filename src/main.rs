use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    process::Command,
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
        tl;dr: Package can't have capitals, underscores, or dashes
        "#
        );
    }

    let mut version: String = prompt("Version (1.0.0): ");
    let regex: Regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    if version.len() == 0 {
        version = "1.0.0".to_string();
    }
    if !regex.is_match(&version) {
        println!("Invalid version provided, defaulting to 1.0.0");
        version = "1.0.0".to_string();
    }
    println!("Initializing package.json... (./{}/package.json)", package);

    let package_json: String = format!(
        r#"{{
  "name": "{}",
  "version": "{}",
  "license": "{}",
  "author": "{}",
  "description": "",
  "main": "src/main.ts",
  "scripts": {{
    "dev": "nodemon --config nodemon.json src/app.ts",
    "start": "node src/app.ts"
  }},
  "dependencies": {{
    "pino": "^8.1.0"
  }},
  "devDependencies": {{
    "@types/express": "^4.17.13",
    "@types/node": "^18.0.0",
    "@types/node-fetch": "^2.6.2",
    "pino-pretty": "^8.1.0",
    "ts-node": "^10.8.1",
    "typescript": "^4.7.4"
  }}
}}"#,
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
    let app_ts: String = r#"
import logger from "./utils/logger";
logger.info("Hello World!");
"#
    .to_string();
    app_ts_file.write_all(app_ts.as_bytes()).unwrap();
    println!("app.ts file created");
}

fn logger_init(package: String) {
    println!("Initializing logger... (./{}/src/utils/logger.ts)", package);
    // mkdir ./package/src/utils
    fs::create_dir_all(format!("./{}/src/utils", package)).unwrap();
    let mut logger_file: File = File::create(format!("./{}/src/utils/logger.ts", package)).unwrap();
    let logger_ts: String = r#"import pino from 'pino';

const logger = pino({
  transport: {
    target: 'pino-pretty'
  },
  customPrettifiers: {
    time: (timestamp : any) => `🕰 ${timestamp}`,
  },
});


export default logger;
    "#
    .to_string();
    logger_file.write_all(logger_ts.as_bytes()).unwrap();
    println!("logger.ts file created");
}

fn tsconfig_json_init(package: String) {
    println!("Initializing tsconfig... (./{}/tsconfig.json)", package);
    let mut tsconfig_json_file: File =
        File::create(format!("./{}/tsconfig.json", package)).unwrap();
    let tsconfig_json: String = r#"{
  "compilerOptions": {
    "resolveJsonModule": true,
    /* Visit https://aka.ms/tsconfig to read more about this file */
  
    /* Projects */
    // "incremental": true,                              /* Save .tsbuildinfo files to allow for incremental compilation of projects. */
    // "composite": true,                                /* Enable constraints that allow a TypeScript project to be used with project references. */
    // "tsBuildInfoFile": "./.tsbuildinfo",              /* Specify the path to .tsbuildinfo incremental compilation file. */
    // "disableSourceOfProjectReferenceRedirect": true,  /* Disable preferring source files instead of declaration files when referencing composite projects. */
    // "disableSolutionSearching": true,                 /* Opt a project out of multi-project reference checking when editing. */
    // "disableReferencedProjectLoad": true,             /* Reduce the number of projects loaded automatically by TypeScript. */
  
    /* Language and Environment */
    "target": "es2016",                                  /* Set the JavaScript language version for emitted JavaScript and include compatible library declarations. */
    // "lib": [],                                        /* Specify a set of bundled library declaration files that describe the target runtime environment. */
    // "jsx": "preserve",                                /* Specify what JSX code is generated. */
    // "experimentalDecorators": true,                   /* Enable experimental support for TC39 stage 2 draft decorators. */
    // "emitDecoratorMetadata": true,                    /* Emit design-type metadata for decorated declarations in source files. */
    // "jsxFactory": "",                                 /* Specify the JSX factory function used when targeting React JSX emit, e.g. 'React.createElement' or 'h'. */
    // "jsxFragmentFactory": "",                         /* Specify the JSX Fragment reference used for fragments when targeting React JSX emit e.g. 'React.Fragment' or 'Fragment'. */
    // "jsxImportSource": "",                            /* Specify module specifier used to import the JSX factory functions when using 'jsx: react-jsx*'. */
    // "reactNamespace": "",                             /* Specify the object invoked for 'createElement'. This only applies when targeting 'react' JSX emit. */
    // "noLib": true,                                    /* Disable including any library files, including the default lib.d.ts. */
    // "useDefineForClassFields": true,                  /* Emit ECMAScript-standard-compliant class fields. */
    // "moduleDetection": "auto",                        /* Control what method is used to detect module-format JS files. */
  
    /* Modules */
    "module": "commonjs",                                /* Specify what module code is generated. */
    // "rootDir": "./",                                  /* Specify the root folder within your source files. */
    // "moduleResolution": "node",                       /* Specify how TypeScript looks up a file from a given module specifier. */
    // "baseUrl": "./",                                  /* Specify the base directory to resolve non-relative module names. */
    // "paths": {},                                      /* Specify a set of entries that re-map imports to additional lookup locations. */
    // "rootDirs": [],                                   /* Allow multiple folders to be treated as one when resolving modules. */
    // "typeRoots": [],                                  /* Specify multiple folders that act like './node_modules/@types'. */
    // "types": [],                                      /* Specify type package names to be included without being referenced in a source file. */
    // "allowUmdGlobalAccess": true,                     /* Allow accessing UMD globals from modules. */
    // "moduleSuffixes": [],                             /* List of file name suffixes to search when resolving a module. */
    // "resolveJsonModule": true,                        /* Enable importing .json files. */
    // "noResolve": true,                                /* Disallow 'import's, 'require's or '<reference>'s from expanding the number of files TypeScript should add to a project. */
  
    /* JavaScript Support */
    // "allowJs": true,                                  /* Allow JavaScript files to be a part of your program. Use the 'checkJS' option to get errors from these files. */
    // "checkJs": true,                                  /* Enable error reporting in type-checked JavaScript files. */
    // "maxNodeModuleJsDepth": 1,                        /* Specify the maximum folder depth used for checking JavaScript files from 'node_modules'. Only applicable with 'allowJs'. */
  
    /* Emit */
    // "declaration": true,                              /* Generate .d.ts files from TypeScript and JavaScript files in your project. */
    // "declarationMap": true,                           /* Create sourcemaps for d.ts files. */
    // "emitDeclarationOnly": true,                      /* Only output d.ts files and not JavaScript files. */
    // "sourceMap": true,                                /* Create source map files for emitted JavaScript files. */
    // "outFile": "./",                                  /* Specify a file that bundles all outputs into one JavaScript file. If 'declaration' is true, also designates a file that bundles all .d.ts output. */
    // "outDir": "./",                                   /* Specify an output folder for all emitted files. */
    // "removeComments": true,                           /* Disable emitting comments. */
    // "noEmit": true,                                   /* Disable emitting files from a compilation. */
    // "importHelpers": true,                            /* Allow importing helper functions from tslib once per project, instead of including them per-file. */
    // "importsNotUsedAsValues": "remove",               /* Specify emit/checking behavior for imports that are only used for types. */
    // "downlevelIteration": true,                       /* Emit more compliant, but verbose and less performant JavaScript for iteration. */
    // "sourceRoot": "",                                 /* Specify the root path for debuggers to find the reference source code. */
    // "mapRoot": "",                                    /* Specify the location where debugger should locate map files instead of generated locations. */
    // "inlineSourceMap": true,                          /* Include sourcemap files inside the emitted JavaScript. */
    // "inlineSources": true,                            /* Include source code in the sourcemaps inside the emitted JavaScript. */
    // "emitBOM": true,                                  /* Emit a UTF-8 Byte Order Mark (BOM) in the beginning of output files. */
    // "newLine": "crlf",                                /* Set the newline character for emitting files. */
    // "stripInternal": true,                            /* Disable emitting declarations that have '@internal' in their JSDoc comments. */
    // "noEmitHelpers": true,                            /* Disable generating custom helper functions like '__extends' in compiled output. */
    // "noEmitOnError": true,                            /* Disable emitting files if any type checking errors are reported. */
    // "preserveConstEnums": true,                       /* Disable erasing 'const enum' declarations in generated code. */
    // "declarationDir": "./",                           /* Specify the output directory for generated declaration files. */
    // "preserveValueImports": true,                     /* Preserve unused imported values in the JavaScript output that would otherwise be removed. */
  
    /* Interop Constraints */
    // "isolatedModules": true,                          /* Ensure that each file can be safely transpiled without relying on other imports. */
    // "allowSyntheticDefaultImports": true,             /* Allow 'import x from y' when a module doesn't have a default export. */
    "esModuleInterop": true,                             /* Emit additional JavaScript to ease support for importing CommonJS modules. This enables 'allowSyntheticDefaultImports' for type compatibility. */
    // "preserveSymlinks": true,                         /* Disable resolving symlinks to their realpath. This correlates to the same flag in node. */
    "forceConsistentCasingInFileNames": true,            /* Ensure that casing is correct in imports. */
  
    /* Type Checking */
    "strict": true,                                      /* Enable all strict type-checking options. */
    // "noImplicitAny": true,                            /* Enable error reporting for expressions and declarations with an implied 'any' type. */
    // "strictNullChecks": true,                         /* When type checking, take into account 'null' and 'undefined'. */
    // "strictFunctionTypes": true,                      /* When assigning functions, check to ensure parameters and the return values are subtype-compatible. */
    // "strictBindCallApply": true,                      /* Check that the arguments for 'bind', 'call', and 'apply' methods match the original function. */
    // "strictPropertyInitialization": true,             /* Check for class properties that are declared but not set in the constructor. */
    // "noImplicitThis": true,                           /* Enable error reporting when 'this' is given the type 'any'. */
    // "useUnknownInCatchVariables": true,               /* Default catch clause variables as 'unknown' instead of 'any'. */
    // "alwaysStrict": true,                             /* Ensure 'use strict' is always emitted. */
    // "noUnusedLocals": true,                           /* Enable error reporting when local variables aren't read. */
    // "noUnusedParameters": true,                       /* Raise an error when a function parameter isn't read. */
    // "exactOptionalPropertyTypes": true,               /* Interpret optional property types as written, rather than adding 'undefined'. */
    // "noImplicitReturns": true,                        /* Enable error reporting for codepaths that do not explicitly return in a function. */
    // "noFallthroughCasesInSwitch": true,               /* Enable error reporting for fallthrough cases in switch statements. */
    // "noUncheckedIndexedAccess": true,                 /* Add 'undefined' to a type when accessed using an index. */
    // "noImplicitOverride": true,                       /* Ensure overriding members in derived classes are marked with an override modifier. */
    // "noPropertyAccessFromIndexSignature": true,       /* Enforces using indexed accessors for keys declared using an indexed type. */
    // "allowUnusedLabels": true,                        /* Disable error reporting for unused labels. */
    // "allowUnreachableCode": true,                     /* Disable error reporting for unreachable code. */
  
    /* Completeness */
    // "skipDefaultLibCheck": true,                      /* Skip type checking .d.ts files that are included with TypeScript. */
    "skipLibCheck": true                                 /* Skip type checking all .d.ts files. */
  }
}
      "#
    .to_string();
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

    let nodemon_json = r#"{
  "restartable": "rs",
  "ignore": [".git", "node_modules/", "dist/", "coverage/"],
  "watch": ["src/"],
  "execMap": {
    "ts": "node -r ts-node/register"
  },
  "env": {
    "NODE_ENV": "development"
  },
  "ext": "js,json,ts"
}"#
    .to_string();
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

    let mit_license = format!(
        r#"MIT License

Copyright (c) {} {}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
        current_year, author
    )
    .to_string();
    let mut license_file = File::create(format!("./{}/LICENSE", package)).unwrap();
    license_file.write_all(mit_license.as_bytes()).unwrap();
    println!("LICENSE created");
}

fn readme_init(package: String) {
    println!("Initializing README... (./{}/README.md)", package);
    let readme = r#"# APP NAME
### App initialized by @alexng353/typescript-generator
## Development
 - Run `npm run dev` to start development.
## Production
 - Run `npm run start` to start the app.
    "#
    .to_string();
    let mut readme_file = File::create(format!("./{}/README.md", package)).unwrap();
    readme_file.write_all(readme.as_bytes()).unwrap();
    println!("README created");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut yes = false;
    // see if there is a -y flag
    for arg in args.iter() {
        if arg == "-y" {
            yes = true;
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

    let package: String = prompt("Package name: ");
    if package.len() == 0 {
        panic!("No package name provided");
    }
    let author: String = prompt("Author: ");
    if author.len() == 0 {
        panic!("No author name provided");
    }
    // mkdir ./EduBeyond
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

    let dir = env::current_dir().unwrap();
    println!("Installing dependencies...");

    if cfg!(target_os = "windows") {
        let mut child = Command::new("cmd")
            .arg("/c")
            .arg("npm install")
            .current_dir(&dir.join(&package))
            .spawn()
            .unwrap();
        child.wait().unwrap();
    } else {
        let mut child = Command::new("npm")
            .arg("install")
            .current_dir(&dir.join(&package))
            .spawn()
            .unwrap();
        child.wait().unwrap();
    };
    println!("{}", format!("\x1b[32m{}\x1b[0m", "\n\nDependencies installed\n\n"));


    println!("Package {} created successfully!", package);
    // red print that says "You need to install nodemon if you don't have it"
    // blue print that says "npm i -g nodemon"
    let blue_text: String = "\x1b[34m".to_string();
    let red_text: String = "\x1b[31m".to_string();
    let end_code: String = "\x1b[0m".to_string();
    println!();
    println!();
    // println!("You need to install nodemon if you don't have it");
    println!(
        "{}You need to install nodemon if you don't have it{}",
        blue_text, end_code
    );
    println!("{}npm i -g nodemon{}", red_text, end_code);
}
