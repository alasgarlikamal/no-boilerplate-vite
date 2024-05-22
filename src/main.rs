use colored::Colorize;
use std::fs::{self, File, OpenOptions};
use std::io::{Result, Write};
use std::process::Output;
use std::{env, process};

fn main() -> Result<()> {
    // Getting project name
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "{} no-boilerplate-vite requires a project name.",
            "ERROR:".red()
        );
        process::exit(1);
    }

    // Check for help option
    match args.get(1) {
        Some(x) => match x.as_str() {
            "-h" => {
                print_usage();
                return Ok(());
            }
            "--help" => {
                print_usage();
                return Ok(());
            }
            _ => (),
        },
        None => (),
    }

    let project_name = &args[1];

    if project_name.contains(".") || project_name.contains("/") || project_name.contains("\\") {
        eprintln!("{} Invalid project name", "ERROR:".red());
        process::exit(1);
    }

    // Check for install argument
    let will_install_npm = match args.get(2) {
        Some(x) => match x.as_str() {
            "-i" => true,
            "--install" => true,
            rest => {
                eprintln!(
                    "{} Unknown arguments passed: [{}]",
                    "ERROR:".red(),
                    rest.yellow()
                );
                process::exit(1);
            }
        },
        None => false,
    };

    let vite_command = format!(
        "npm create vite@latest {} -- --template react-ts",
        &project_name
    );

    print_command(&args);
    print_logo();
    println!(
        "{} Bootstrapping {} project -> {}",
        "LOG:".green(),
        "VITE".yellow(),
        "TS + Tailwind".blue()
    );
    // Creating vite project
    process::Command::new("sh")
        .arg("-c")
        .arg(vite_command)
        .output()
        .expect("Failed to execute");

    env::set_current_dir(project_name).unwrap();

    println!("{} Removing boilerplate", "LOG:".green());
    // Rewrite readme
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("README.md")?;

    let readme_content = format!("# {}", &project_name);
    f.write_all(readme_content.as_bytes())?;
    f.flush()?;

    // Replace Title in index.html
    let mut html = fs::read_to_string("index.html")?;
    html = html.replace("Vite + React + TS", &project_name);

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("index.html")?;
    f.write_all(html.as_bytes())?;

    // Replace App.jsx with tailwind code
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("src/App.tsx")?;

    let new_app_content = "export default function App() {
return (
    <h1 className=\"text-3xl p-2 font-bold underline\">
        Everything works!
    </h1>
    )
}";

    f.write_all(new_app_content.as_bytes())?;
    f.flush()?;

    // Remove App.css
    fs::remove_file("src/App.css")?;

    // Remove react.svg
    fs::remove_file("src/assets/react.svg")?;

    println!("{} Creating {} config", "LOG:".green(), "Tailwind".blue());
    // Create tailwind.config.js
    let mut f = File::create("tailwind.config.js")?;

    let config = b"/** @type {import('tailwindcss').Config} */
export default {
    content: [
        \"./index.html\",
        \"./src/**/*.{js,ts,jsx,tsx}\",
      ],
    theme: {
        extend: {},
    },
    plugins: [],
}";

    f.write_all(config)?;
    f.flush()?;

    println!("{} Creating {} config", "LOG:".green(), "Postcss".red());
    // Create postcss.config.js
    let mut f = File::create("postcss.config.js")?;

    let config = b"export default {
    plugins: {
    tailwindcss: {},
    autoprefixer: {},
    },
}";

    f.write_all(config)?;
    f.flush()?;

    // Edit index.css
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("src/index.css")?;

    let content = b"@tailwind base;
@tailwind components;
@tailwind utilities;";
    f.write_all(content)?;
    f.flush()?;

    // Check for install argument
    if will_install_npm {
        println!("{} Installing {} packages", "LOG:".green(), "NPM".red());
        install_packages()?;
        println!("{}", "Successfully scaffolded project".green());
        println!("  cd {}\n  npm run dev", project_name);
        return Ok(());
    }
    println!("{}", "Successfully scaffolded project".green());
    println!(
        "  cd {}\n  npm install -D tailwindcss postcss autoprefixer\n  npm run dev",
        project_name
    );
    Ok(())
}

fn install_packages() -> Result<Output> {
    return process::Command::new("sh")
        .arg("-c")
        .arg("npm install -D tailwindcss postcss autoprefixer")
        .output();
}

fn print_usage() {
    println!("Usage: nvb [project-name] [options]");
    println!("Options:");
    println!("  --install, -i          Install npm packages too");
    println!("  --help, -h             Print command line options");
}

fn print_logo() {
    println!("         _");
    println!(" _ __   | |__   __   __");
    println!("| '_ \\  | '_ \\  \\ \\ / /");
    println!("| | | | | |_) |  \\ V /");
    println!("|_| |_| |_.__/    \\_/");
    println!("_______________________");
    println!("< No Boilerplate Vite >");
    println!("-----------------------");
}

fn print_command(args: &Vec<String>) {
    print!("> ");
    for arg in args {
        print!("{arg} ");
    }
    println!("");
}
