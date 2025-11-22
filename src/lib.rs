use std::{error::Error, path::PathBuf, process};

pub mod cli;
pub mod config;
pub mod picker;
pub mod session;
pub mod source;
pub mod tmux;
pub mod utils;

pub fn run(args: cli::Args) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    println!("Running sessionizer with args: {:?}", args);

    // Default config location should be XDG_CONFIG_HOME/tmux-sessionizer/config.toml or the result
    // of args.config_path
    let config_location = match args.config_path {
        Some(ref path) => PathBuf::from(path),
        None => config::default_config_path(),
    };

    let config = config::Config::from_file(&config_location)?;
    let session_manager = session::manager::SessionManager::new();
    let source_provider = source::provider::SourceProvider::new(&session_manager, &config);

    #[cfg(debug_assertions)]
    let _ = dbg!(&config);

    match args.name {
        Some(name) => {
            // If name is a directory path, take the head as session name and create or attach
            let path = PathBuf::from(&name);
            if path.is_dir() {
                let session_name = path
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap_or("default_session");
                session::create_or_attach(session_name, Some(&path), false).unwrap();
            } else {
                session::create_or_attach(&name, None, false).unwrap();
            }
        }
        None => {
            #[cfg(debug_assertions)]
            println!("Opening fuzzy picker...");

            let sources = source_provider.gather_sources()?;

            #[cfg(debug_assertions)]
            {
                for source in &sources {
                    println!("{}", source.display_name(config.display_tilde));
                }
            }

            if sources.is_empty() {
                eprintln!("No tmux sessions found to pick from.");
                process::exit(1);
            }

            match picker::fuzzy_pick_session(sources) {
                Some(selected) => {
                    use source::Source;
                    match selected.clone() {
                        Source::TmuxSession(name) => {
                            session::create_or_attach(&name, None, false).unwrap();
                        }
                        Source::Directory(path) => {
                            let session_name: &str = &selected.session_name();
                            session::create_or_attach(session_name, Some(&path), false).unwrap();
                        }
                    }
                }
                None => {
                    #[cfg(debug_assertions)]
                    eprintln!("No session selected, exiting.");
                    process::exit(1);
                }
            }
        }
    }

    Ok(())
}
