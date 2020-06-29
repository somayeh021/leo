use crate::{
    cli::*,
    cli_types::*,
    commands::BuildCommand,
    errors::CLIError,
    files::{ChecksumFile, Manifest, ProofFile, ProvingKeyFile, VerificationKeyFile},
};

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct CleanCommand;

impl CLI for CleanCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Clean the outputs directory";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "clean";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let (_program, _checksum_differs) = BuildCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        // Remove the checksum from the outputs directory
        ChecksumFile::new(&package_name).remove(&path)?;

        // Remove the proving key from the outputs directory
        ProvingKeyFile::new(&package_name).remove(&path)?;

        // Remove the verification key from the outputs directory
        VerificationKeyFile::new(&package_name).remove(&path)?;

        // Remove the proof from the outputs directory
        ProofFile::new(&package_name).remove(&path)?;

        Ok(())
    }
}
