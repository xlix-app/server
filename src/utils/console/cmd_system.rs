use sessionless::hex::FromHex;
use sessionless::PublicKey;
use crate::database::Database;
use crate::error::RHSError::*;
use super::*;

static ERR_MSG_MISSING_ACTION: &str = "Missing argument [0 - action] from: system <action>";
static ERR_MSG_INVALID_ACTION: &str = "Invalid argument [0 - action] from: system <action>";
static ERR_MSG_MISSING_CREATE_NAME: &str = "Missing argument [1 - name] from: system create <name>";

pub struct System;

impl System {
    async fn create<'a>(ins: Instruction<'a>) -> Output {
        let name = *ins.args.get(1).ok_or(ERR_MSG_MISSING_ACTION)?;
        let mut public_key = None;

        if let Some(public_key_hex) = ins.args.get(2) {
            public_key = Some(
                PublicKey::from_hex(*public_key_hex)
                    .map_err(|err|
                        format!("Invalid argument [2 - public_key] from: system create <name> <public_key [{}]>", err)
                    )?
            );
        }

        let db: &Database = Database::get().await.unwrap();

        let system = db
            .system_create(name, public_key)
            .await
            .map_err(|err| format!("Execution error: [{}]", err))?;

        Ok(format!(
            "Success! Created [{}] system with ID of [{}]",
            system.name,
            system.id.into_raw()
        ).into())
    }
}

impl CommandInfo for System {
    fn caller(&self) -> &'static str {
        "system"
    }
}

impl Command for System {
    type Output = Output;

    fn on_execute<'a>(&self, ins: Instruction<'a>) -> OutputFuture<'a, Self::Output> {
        async move {
            let action = *ins.args.get(0).ok_or(ERR_MSG_MISSING_ACTION)?;
            return match action {
                "create" => System::create(ins).await,
                _ => Err(ERR_MSG_INVALID_ACTION.into())
            };
        }.output_future()
    }
}
