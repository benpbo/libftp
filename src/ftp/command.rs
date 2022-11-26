use std::net::Ipv4Addr;

// FTP commands according to RFC 959
pub enum Command<'a> {
    // Access control
    UserName(&'a [u8]),
    Password(&'a [u8]),
    Account(&'a [u8]),
    ChangeWorkingDirectory(&'a [u8]),
    ChangeToParentDirectory,
    StructureMount(&'a [u8]),
    Reinitialize,
    Logout,

    // Transfer parameter
    DataPort(Ipv4Addr, u16),
    Passive,
    RepresentationType(RepresentationTypeKind),
    FileStructure(FileStructureKind),
    TransferMode(TransferModeKind),

    // FTP service
    Retrieve(&'a [u8]),
    Store(&'a [u8]),
    StoreUnique,
    Append(&'a [u8]),
    Allocate(i64, Option<i64>),
    Restart(&'a [u8]),
    RenameFrom(&'a [u8]),
    RenameTo(&'a [u8]),
    Abort,
    Delete(&'a [u8]),
    RemoveDirectory(&'a [u8]),
    MakeDirectory(&'a [u8]),
    PrintWorkingDirectory,
    List(Option<&'a [u8]>),
    NameList(Option<&'a [u8]>),
    SiteParameters(&'a [u8]),
    System,
    Status(Option<&'a [u8]>),
    Help(Option<&'a [u8]>),
    Noop,
}

pub enum FormatControl {
    NonPrint,
    Telnet,
    Carriage,
}

pub enum RepresentationTypeKind {
    Ascii(Option<FormatControl>),
    Ebcdic(Option<FormatControl>),
    Image,
    LocalByte(u8),
}

pub enum FileStructureKind {
    File,
    Record,
    Page,
}

pub enum TransferModeKind {
    Stream,
    Block,
    Compressed,
}
