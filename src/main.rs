use std::iter;

use structopt::StructOpt;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use colored::*;

use verve::Verve;
use verve::Word;
use verve::anagram::AnagramR;

#[derive(StructOpt)]
struct VerveCLI {
    #[structopt(short, long)]
    dictionary : Option<String>,
    #[structopt(subcommand)]
    command : Option<VerveCommand>,
}

#[derive(StructOpt)]
enum VerveCommand {
    #[structopt(name = "anagram")]
    Anagram { 
        #[structopt(short, long)]
        exact : bool,
        word : Word,
    },
    #[structopt(name = "multigram")]
    Multigram { 
        word : Word,
        #[structopt(short, long)]
        limit : Option<usize>
    },
    #[structopt(name = "quit")]
    Quit,
}

#[derive(PartialEq, Eq)]
enum InteractiveStatus {
    Continue, Stop
}

#[derive(Default)]
struct Performers {
    anagramr : Option<AnagramR> 
}

fn execute_command(verve : &Verve, performers : &mut Performers, command : VerveCommand) -> InteractiveStatus {
    match command {
        VerveCommand::Anagram { exact, word } => {
            let anagramr = performers.anagramr.get_or_insert_with(
                || {
                    println!("{}", "... building anagram capabilites".cyan().bold());
                    AnagramR::new(&verve)
                }
            );
            let result_ids : Vec<verve::Id>;
            if exact {
                result_ids = anagramr.exact_anagrams(&word);
            } else {
                result_ids = anagramr.anagrams(&word);
            }
            let mut out_vec : Vec<String> = result_ids
                .iter()
                .map(|id| verve.word(*id))
                .map(|w| format!("{}", w))
                .collect();
            out_vec.sort_unstable_by(
                |a, b| (-1 * a.len() as isize, a).cmp(&(-1 * b.len() as isize, b))
            );

            println!("{:?}", out_vec);
            println!("{} anagrams found", out_vec.len());
        }
        VerveCommand::Multigram { word , limit } => {
            let anagramr = performers.anagramr.get_or_insert_with(
                || {
                    println!("{}", "... building anagram capabilites".cyan().bold());
                    AnagramR::new(&verve)
                }
            );
            let result_vecs = anagramr.multigrams(&word, limit);
            let out_vec : Vec<Vec<String>> = result_vecs
                .iter()
                .map(|ids| ids.into_iter()
                    .map(|id| verve.word(*id))
                    .map(|w| format!("{}", w))
                    .collect()
                ).collect();

            println!("{:?}", out_vec);
            println!("{} multigrams found", out_vec.len());
        }
        VerveCommand::Quit => { 
            return InteractiveStatus::Stop;
        }
    }
    return InteractiveStatus::Continue;
}

fn main() {
    let cli = VerveCLI::from_args();
    let mut performers : Performers = Default::default();
    let verve = match cli.dictionary {
        None => Verve::new(),
        Some(dict_name) => Verve::new_from(&dict_name)
    };
    match cli.command {
        None => {
            //interactive mode
            let mut rl = Editor::<()>::new();
            let _history = rl.load_history("history.txt");
            let mut status = InteractiveStatus::Continue;
            while status == InteractiveStatus::Continue {
                let readline = rl.readline(&"——⟩ ".bold().green().to_string());
                status = match readline {
                    Ok(line) => {
                        let input = line.as_str();
                        rl.add_history_entry(input);
                        let command_result = VerveCommand::from_iter_safe(
                            iter::once(" ").chain(input.split(" "))
                        );
                        match command_result {
                            Ok(command) => execute_command(&verve, &mut performers, command),
                            Err(err) => {
                                println!("{}", err);
                                InteractiveStatus::Continue
                            }
                        }
                    },
                    Err(ReadlineError::Interrupted) => InteractiveStatus::Stop,
                    Err(ReadlineError::Eof) => InteractiveStatus::Stop,
                    _ => InteractiveStatus::Continue,
                };
            }
            rl.save_history("history.txt").unwrap();
        }
        Some(command) => {
            //direct command mode
            execute_command(&verve, &mut performers, command);
        }
    }
}


