use std::io::Write;

use crate::ftp::Command;

use super::serializeable::Serializeable;
use super::Serializer;

pub struct CommandSerializer<W: Write> {
    writer: W,
}

impl<W: Write> CommandSerializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn serialize_command(&mut self, value: &Command) -> std::io::Result<()> {
        macro_rules! serialize {
            ($command: tt) => {{
                stringify!($command)
                    .as_bytes()
                    .serialize(&mut self.writer)?;
            }};
            ($command: tt, $value: expr) => {{
                serialize!($command);
                b" ".serialize(&mut self.writer)?;
                $value.serialize(&mut self.writer)?;
            }};
        }

        match value {
            Command::UserName(username) => serialize!(USER, username),
            Command::Password(password) => serialize!(PASS, password),
            Command::Account(account) => serialize!(ACCT, account),
            Command::ChangeWorkingDirectory(pathname) => serialize!(CWD, pathname),
            Command::ChangeToParentDirectory => serialize!(CDUP),
            Command::StructureMount(pathname) => serialize!(SMNT, pathname),
            Command::Reinitialize => serialize!(REIN),
            Command::Logout => serialize!(QUIT),
            Command::DataPort(address, port) => serialize!(PORT, (*address, *port)),
            Command::Passive => serialize!(PASV),
            Command::RepresentationType(kind) => serialize!(TYPE, kind),
            Command::FileStructure(kind) => serialize!(STRU, kind),
            Command::TransferMode(kind) => serialize!(MODE, kind),
            Command::Retrieve(pathname) => serialize!(RETR, pathname),
            Command::Store(pathname) => serialize!(STOR, pathname),
            Command::StoreUnique => serialize!(STOU),
            Command::Append(pathname) => serialize!(APPE, pathname),
            Command::Allocate(reserve, maximum_size) => {
                serialize!(ALLO, reserve);
                if let Some(size) = maximum_size {
                    b" R ".serialize(&mut self.writer)?;
                    size.serialize(&mut self.writer)?;
                }
            }
            Command::Restart(marker) => serialize!(REST, marker),
            Command::RenameFrom(pathname) => serialize!(RNFR, pathname),
            Command::RenameTo(pathname) => serialize!(RNTO, pathname),
            Command::Abort => serialize!(ABOR),
            Command::Delete(pathname) => serialize!(DELE, pathname),
            Command::RemoveDirectory(pathname) => serialize!(RMD, pathname),
            Command::MakeDirectory(pathname) => serialize!(MKD, pathname),
            Command::PrintWorkingDirectory => serialize!(PWD),
            Command::List(pathname) => serialize!(LIST, pathname),
            Command::NameList(pathname) => serialize!(NLST, pathname),
            Command::SiteParameters(parameters) => serialize!(SITE, parameters),
            Command::System => serialize!(SYST),
            Command::Status(pathname) => serialize!(STAT, pathname),
            Command::Help(command) => serialize!(HELP, command),
            Command::Noop => serialize!(NOOP),
        }

        self.writer.write_all(b"\r\n")
    }
}

impl<W: Write> Serializer<Command> for CommandSerializer<W> {
    fn serialize(&mut self, value: &Command) -> std::io::Result<()> {
        self.serialize_command(value)?;
        self.writer.flush()
    }
}
