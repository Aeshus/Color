use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Cli {
    pub file_path: Option<PathBuf>,
    pub display_options: DisplayOptions,
}

#[derive(Debug)]
pub struct DisplayOptions {
    descriptive: Option<bool>,
    color: Option<bool>,
}

impl From<std::env::Args> for Cli {
    fn from(args: std::env::Args) -> Self {
        // Skip first one, as that's always the command name.

        let mut cli = Cli {
            file_path: None,
            display_options: DisplayOptions {
                descriptive: None,
                color: None,
            },
        };

        // Iterate over the remaining env variables
        for mut argument in args.skip(1) {
            // We're able to use unwrap here because the iterator won't
            // read past it's bounds.
            if argument.chars().nth(0).unwrap() == '-' {
                let mut dash_num = 0;

                for dash_char in argument.chars() {
                    if dash_char == '-' {
                        dash_num += 1;
                    }
                }

                for _ in 0..dash_num {
                    argument.remove(0);
                }

                // Sanity Check
                if dash_num == 0 {
                    panic!()
                }

                // Short-Hand
                if dash_num == 1 {
                    for chars in argument.chars() {
                        match chars {
                            'd' => {
                                if cli.display_options.descriptive == None {
                                    cli.display_options.descriptive = Some(true);
                                } else {
                                    println!(
                                        "Don't assign multiple discription options. Ignoring {} option.",
                                        chars
                                    );
                                }
                            }
                            'c' => {
                                if cli.display_options.color == None {
                                    cli.display_options.color = Some(true);
                                } else {
                                    println!(
                                        "Don't assign multiple color options. Ignoring {} option.",
                                        chars
                                    );
                                }
                            }
                            _ => {
                                println!("Unknown short flag: '{}'", chars);
                            }
                        }
                    }
                }

                // Long-Hand
                if dash_num >= 2 {
                    match argument.as_str() {
                        "description" | "descriptive" => {
                            if cli.display_options.descriptive == None {
                                cli.display_options.descriptive = Some(true);
                                continue;
                            }
                            println!(
                                "Don't assign multiple description options. Ignoring {} option.",
                                argument
                            );
                        }
                        "color" | "colorful" => {
                            if cli.display_options.color == None {
                                cli.display_options.color = Some(true);
                                continue;
                            }
                            println!(
                                "Don't assign multiple color options. Ignoring {} option.",
                                argument
                            );
                        }
                        _ => {
                            println!("Unknown long flag: '{}'", argument)
                        }
                    }
                }

                continue;
            }

            if cli.file_path != None {
                println!("Don't assign multiple paths. Ignoring '{}'", argument);
                continue;
            }

            let temp_path = Path::new(&argument);

            if !(temp_path.exists()) {
                println!("Path Supplied ({:?}) does not exist", temp_path);
                continue;
            }

            if !(temp_path.extension().unwrap() == "png") {
                println!("File supplied ({:?}) is not a PNG", temp_path);
                continue;
            }

            cli.file_path = Some(PathBuf::from(argument));
        }

        if cli.display_options.descriptive == None {
            cli.display_options.descriptive = Some(false);
        }

        if cli.display_options.color == None {
            cli.display_options.color = Some(false);
        }

        cli
    }
}

// impl Cli {
//     pub fn from_vec(input: Vec<String>) -> Cli {
//         let mut output: Option<Output> = None;
//         let mut path: Option<PathBuf> = None;

//         for (index, argument) in input.iter().enumerate().skip(1) {
//             // Clone's v & turns into string.
//             // Done to allow parts of the string to be removed over time.
//             // Also makes sure that the final output is still fine.
//             let mut string = argument.to_string();

//             println!("{}", argument);

//             if string.chars().nth(0).unwrap() == '-' {
//                 match output {
//                     Some(..) => {
//                         println!(
//                             "WARNING: Do not input multiple output methods!
//                             Skipping {}; Using {:?}",
//                             string, output
//                         );
//                         break;
//                     }
//                     _ => (),
//                 }

//                 // Remove preceeding dashes to make patern maching easier
//                 string.remove(0);
//                 if string.chars().nth(0).unwrap() == '-' {
//                     string.remove(0);
//                 }

//                 match string.as_str() {
//                     "d" | "descriptive" => {
//                         output = Some(Output::Descriptive);
//                     }
//                     "n" | "normal" | _ => {
//                         output = Some(Output::Normal);
//                     }
//                     _ => {}
//                 }
//             } else {
//                 // Sanity check
//                 // Makes sure there aren't multiple paths inputted
//                 match path {
//                     Some(..) => {
//                         println!(
//                             "WARNING: Do not input multiple paths!
//                             Skipping {}; Using {:?}",
//                             argument, path
//                         );
//                         break;
//                     }
//                     _ => (),
//                 }

//                 // Checks to make sure the path exists.
//                 if Path::new(&input[index]).exists() {
//                     path = Some(PathBuf::from(v));
//                 }
//             }
//         }

//         return Cli { path, output };
//     }
// }
