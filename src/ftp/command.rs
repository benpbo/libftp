use std::net::Ipv4Addr;

// FTP commands according to RFC 959
#[derive(Debug)]
pub enum Command {
    // Access control
    UserName(Vec<u8>),
    Password(Vec<u8>),
    Account(Vec<u8>),
    ChangeWorkingDirectory(Vec<u8>),
    ChangeToParentDirectory,
    StructureMount(Vec<u8>),
    Reinitialize,
    Logout,

    // Transfer parameter
    DataPort(Ipv4Addr, u16),
    Passive,
    RepresentationType(RepresentationTypeKind),
    FileStructure(FileStructureKind),
    TransferMode(TransferModeKind),

    // FTP service
    Retrieve(Vec<u8>),
    Store(Vec<u8>),
    StoreUnique,
    Append(Vec<u8>),
    Allocate(i64, Option<i64>),
    Restart(Vec<u8>),
    RenameFrom(Vec<u8>),
    RenameTo(Vec<u8>),
    Abort,
    Delete(Vec<u8>),
    RemoveDirectory(Vec<u8>),
    MakeDirectory(Vec<u8>),
    PrintWorkingDirectory,
    List(Option<Vec<u8>>),
    NameList(Option<Vec<u8>>),
    SiteParameters(Vec<u8>),
    System,
    Status(Option<Vec<u8>>),
    Help(Option<Vec<u8>>),
    Noop,
}

#[derive(Debug)]
pub enum FormatControl {
    NonPrint,
    Telnet,
    Carriage,
}

#[derive(Debug)]
pub enum RepresentationTypeKind {
    Ascii(Option<FormatControl>),
    Ebcdic(Option<FormatControl>),
    Image,
    LocalByte(u8),
}

#[derive(Debug)]
pub enum FileStructureKind {
    File,
    Record,
    Page,
}

#[derive(Debug)]
pub enum TransferModeKind {
    Stream,
    Block,
    Compressed,
}
