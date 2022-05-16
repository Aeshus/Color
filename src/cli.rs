use std::path::Path;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Cli {
    pub file_path: Option<PathBuf>,
    pub display_options: DisplayOptions,
}

#[derive(Debug)]
pub struct DisplayOptions {
    // If description should be added to the stdout messages.
    descriptive: Option<bool>,
    // If the stdout messages should contain color.
    color: Option<bool>,
}

// Takes an Args object and returns a CLI
impl From<std::env::Args> for Cli {
    fn from(args: std::env::Args) -> Self {
        let mut cli = Cli {
            file_path: None,
            display_options: DisplayOptions {
                descriptive: None,
                color: None,
            },
        };

        // Iterate over the remaining env variables
        for mut argument in args.skip(1) {
            if argument.chars().nth(0).unwrap() == '-' {
                let mut dash_num = 0;

                for dash_char in argument.chars() {
                    if !(dash_char == '-') {
                        break;
                    }
                    dash_num += 1;
                }

                for _ in 0..dash_num {
                    argument.remove(0);
                }

                // Short Option Style
                // https://www.gnu.org/software/tar/manual/html_node/Short-Options.html
                // Allows you to concatonate multiple single-letter options together
                if dash_num == 1 {
                    for chars in argument.chars() {
                        match chars {
                            'd' => {
                                if cli.display_options.descriptive.is_some() {
                                    println!(
                                        "Don't assign multiple discription options. Ignoring '{}'.",
                                        chars
                                    );
                                }

                                cli.display_options.descriptive = Some(true);
                            }
                            'c' => {
                                if cli.display_options.color.is_some() {
                                    println!(
                                        "Don't assign multiple color options. Ignoring '{}'.",
                                        chars
                                    );
                                }

                                cli.display_options.color = Some(true);
                            }
                            _ => {
                                println!("Unknown short flag: '{}'", chars);
                            }
                        }
                    }
                }

                // Long Option Style
                // https://www.gnu.org/software/tar/manual/html_node/Long-Options.html
                // It's a more human readable but bigger option method.
                if dash_num >= 2 {
                    match argument.as_str() {
                        "description" | "descriptive" => {
                            if cli.display_options.descriptive.is_some() {
                                println!(
                                    "Don't assign multiple description options. Ignoring {} option.",
                                    argument
                                );
                            }
                            cli.display_options.descriptive = Some(true);
                        }
                        "color" | "colorful" => {
                            if cli.display_options.color.is_some() {
                                println!(
                                    "Don't assign multiple color options. Ignoring {} option.",
                                    argument
                                );
                            }
                            cli.display_options.color = Some(true);
                        }
                        _ => {
                            println!("Unknown long flag: '{}'", argument)
                        }
                    }
                }

                // Skip through rest of the code
                // The path coce could end up running if we don't skip now -- as the program modifies the argument string to make it not have dashes.
                continue;
            }

            // If there's no dashes, assume it's meant to be the path
            if cli.file_path != None {
                println!("Don't assign multiple paths. Ignoring '{}'", argument);
                continue;
            }

            if !(Path::new(&argument).exists()) {
                panic!("Path supplied '{:?}' does not exist", &argument);
            }

            // Resolves path (symlinks, etc) and turns it into a PathBuf
            // Possible unaccounted for error: "A non-final component in path is not a directory.""
            let path = match Path::new(&argument).canonicalize() {
                Ok(x) => x,
                Err(..) => {
                    panic!("Error resolving the path supplied '{:?}'", &argument);
                }
            };

            match verify_file_path(&path) {
                Some(..) => (),
                None => (),
            }

            cli.file_path = Some(path);
        }

        // Default Values
        if cli.display_options.descriptive == None {
            cli.display_options.descriptive = Some(false);
        }

        if cli.display_options.color == None {
            cli.display_options.color = Some(false);
        }

        cli
    }
}

fn verify_file_path(file_path: &PathBuf) -> Option<()> {
    match file_path.extension() {
        Some(x) => {
            if !(x == "png") {
                panic!("Path supplied '{:?}' is not a png", file_path);
            }
        }
        None => {
            panic!(
                "Path supplied '{:?}' does not have a file extension",
                file_path
            );
        }
    }

    Some(())
}
