use fnv::FnvHashMap;

use std::str::FromStr;

use super::Address;
use super::core::debugger;
use super::core::arch::{Arch, Register};

pub struct Command {
    /// The name of the command.
    pub name: &'static str,
    // The help message to be printed by the help command.
    pub help: &'static str,
    // The function to be called when the command is entered.
    pub execute: fn(&[&str], &mut debugger::Debugger) -> i32,
}

impl Command {
    /// Creates and returns a `FnvHashMap` containing the builtin commands.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use std::path::Path;
    /// # use rdbg_core::core::debugger;
    /// use fnv::FnvHashMap;
    /// use rdbg_core::commands;
    ///
    /// # let program = Path::new("./hello_world.bin");
    /// # let mut dbg = debugger::Debugger::new();
    ///
    /// # if let Err(error) = dbg.execute_target(program, &[]) {
    /// #    println!("Error: {}", error);
    /// # }
    ///
    /// let commands: FnvHashMap<&'static str, commands::Command> = commands::Command::map();
    /// if let Some(cmd) = commands.get("continue") {
    ///     (cmd.execute)(&[], &mut dbg);
    /// }
    /// ```
    pub fn map() -> FnvHashMap<&'static str, Self> {
        let mut commands: FnvHashMap<&str, Self> =
            FnvHashMap::with_capacity_and_hasher(32, Default::default());

        // Macro for inserting commands into the command map
        // Credit: https://github.com/redox-os/ion/blob/master/src/builtins/mod.rs
        macro_rules! insert_command {
            ($name:expr, $help:expr, $func:ident) => {
                commands.insert(
                    $name,
                    Command {
                        name: $name,
                        help: $help,
                        execute: $func,
                    }
                );
            }
        }

        insert_command!(
            "continue",
            "Continue program being debugged, after signal or breakpoint.",
            command_continue
        );

        insert_command!(
            "break",
            "Set breakpoint at specified location.",
            command_break
        );

        insert_command!(
            "print",
            "Print value of expression EXP.",
            command_print
        );

        insert_command!(
            "memory",
            "Read or write to process memory.",
            command_memory
        );

        insert_command!(
            "stepi",
            "Step one instruction exactly.",
            command_stepi
        );

        commands
    }
}

//
// TODO: Make these return JSON objects
//
fn command_continue(args: &[&str], dbg: &mut debugger::Debugger) -> i32 {
    info!("Calling continue command");
    dbg.continue_execution();
    0
}

fn command_break(args: &[&str], dbg: &mut debugger::Debugger) -> i32 {
    info!("Calling break command");
    if args.len() < 1 {
        error!("No address given");
        return 1;
    }

    let address = Address::from_str_radix(args[0], 16).unwrap();
    dbg.set_breakpoint_at(address);
    0
}

fn command_print(args: &[&str], dbg: &mut debugger::Debugger) -> i32 {
    println!("RIP: {:?}", format!("{:#x}", dbg.get_register_value(Register::Rip)));
    0
}

fn command_memory(args: &[&str], dbg: &mut debugger::Debugger) -> i32 {
    if args[0] == "read" {
        let address = Address::from_str(args[1]).unwrap();
        println!("{:?}", format!("{:#x}", dbg.read_memory(address).unwrap()));
    }

    if args[0] == "write" {
        dbg.write_memory(Address::from_str(args[1]).unwrap(), i64::from_str(args[2]).unwrap());
    }
    0
}

fn command_stepi(args: &[&str], dbg: &mut debugger::Debugger) -> i32 {
    dbg.single_step_instruction_with_breakpoints();
    0
}
