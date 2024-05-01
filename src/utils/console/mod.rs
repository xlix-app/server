use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use command_engine::*;
use tokio::io::{AsyncBufReadExt, BufReader, stdin};
use tokio::sync::OnceCell;

static COMMAND_ENGINE: OnceCell<Engine> = OnceCell::const_new();

pub type Output = Result<(), ()>;

pub struct Engine {
    console_reader_running: AtomicBool,
    commands: HashMap<&'static str, Box<dyn Command<Output=Output>>>,
}

impl Engine {
    pub async fn init() {
        let engine = COMMAND_ENGINE.get_or_init(Self::new).await;
        tokio::spawn(Self::run_console_reader(engine));
    }

    pub fn get() -> Option<&'static Engine> {
        COMMAND_ENGINE.get()
    }

    pub async fn execute(&self, input: impl AsRef<str>) -> Result<Output, Error> {
        let input = input.as_ref().trim();

        let instruction = Instruction::new(input)?;

        if let Some(command) = self.commands.get(instruction.caller) {
            Ok(command.on_execute(instruction).await)
        } else {
            Err(Error::EngineCommandNotFound)
        }
    }

    async fn new() -> Self {
        let mut map = HashMap::new();

        Self {
            console_reader_running: AtomicBool::new(false),
            commands: map,
        }
    }

    async fn run_console_reader(engine: &'static Engine) {
        if engine.console_reader_running.load(Ordering::SeqCst) == true {
            warn!("Tried to run the console reader, but it was already running!");
            return;
        } else {
            engine.console_reader_running.store(true, Ordering::SeqCst);
        }

        let mut input = String::new();
        let mut reader = BufReader::new(stdin());

        info!("Console runner initialized!");

        while let Ok(_) =  reader.read_line(&mut input).await {
            let instruction = input.trim();

            match engine.execute(instruction).await {
                Ok(Ok(_)) => {
                    info!("Ok!");
                }
                Ok(Err(_)) => {
                    error!("Err!");
                }
                Err(err) => error!("[ENGINE ERROR] {}", err),
            }

            input.clear();
        }

        error!("Console reader has been disabled!");
    }
}
