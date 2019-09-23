use std::iter;

use structopt::StructOpt;
use rustyline::Editor;

use verve::Verve;
use verve::Word;
use verve::anagram::AnagramR;

#[derive(StructOpt)]
struct VerveCLI {
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
                    println!("... building anagram capabilites");
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
        }
        VerveCommand::Multigram { word , limit } => {
            let anagramr = performers.anagramr.get_or_insert_with(
                || {
                    println!("... building anagram capabilites");
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
        }
        VerveCommand::Quit => { 
            return InteractiveStatus::Stop;
        }
    }
    return InteractiveStatus::Continue;
}

fn main() {
    let cli = VerveCLI::from_args();
    let verve = Verve::new();
    let mut performers : Performers = Default::default();
    match cli.command {
        None => {
            //interactive mode
            let mut rl = Editor::<()>::new();
            let mut status = InteractiveStatus::Continue;
            while status == InteractiveStatus::Continue {
                let readline = rl.readline("——⟩ ");
                status = match readline {
                    Ok(line) => {
                        let input = line.as_str();
                        rl.add_history_entry(input);
                        let command = VerveCommand::from_iter(
                            iter::once(" ").chain(input.split(" "))
                        );
                        execute_command(&verve, &mut performers, command)
                    },
                    _ => InteractiveStatus::Stop
                };
            }
        }
        Some(command) => {
            //direct command mode
            execute_command(&verve, &mut performers, command);
        }
    }
}


