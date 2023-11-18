use std::{io::Write, net::Ipv4Addr};

use crate::command::{FileStructureKind, FormatControl, RepresentationTypeKind, TransferModeKind};

pub trait Serializeable {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write;
}

impl Serializeable for (Ipv4Addr, u16) {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        let (address, port) = self;

        // Serialize address
        for octet in address.octets() {
            octet.serialize(writer)?;
            b",".serialize(writer)?;
        }

        // Serialize port
        let [a, b] = port.to_be_bytes();
        a.serialize(writer)?;
        b",".serialize(writer)?;
        b.serialize(writer)
    }
}

impl Serializeable for RepresentationTypeKind {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            RepresentationTypeKind::Ascii(format) => {
                format.serialize(writer)?;
                b"A"
            }
            RepresentationTypeKind::Ebcdic(format) => {
                format.serialize(writer)?;
                b"E"
            }
            RepresentationTypeKind::Image => b"I",
            RepresentationTypeKind::LocalByte(size) => {
                size.serialize(writer)?;
                b"L"
            }
        }
        .serialize(writer)
    }
}

impl Serializeable for FormatControl {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            FormatControl::NonPrint => b"N",
            FormatControl::Telnet => b"T",
            FormatControl::Carriage => b"C",
        }
        .serialize(writer)
    }
}

impl Serializeable for FileStructureKind {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            FileStructureKind::File => b"F",
            FileStructureKind::Record => b"R",
            FileStructureKind::Page => b"P",
        }
        .serialize(writer)
    }
}

impl Serializeable for TransferModeKind {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            TransferModeKind::Stream => b"S",
            TransferModeKind::Block => b"B",
            TransferModeKind::Compressed => b"C",
        }
        .serialize(writer)
    }
}

impl Serializeable for [u8] {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        writer.write_all(self)
    }
}

impl Serializeable for i64 {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        self.to_string().as_bytes().serialize(writer)
    }
}

impl Serializeable for u8 {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        self.to_string().as_bytes().serialize(writer)
    }
}

impl Serializeable for Vec<u8> {
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        self.as_slice().serialize(writer)
    }
}

impl<T> Serializeable for Option<T>
where
    T: Serializeable,
{
    fn serialize<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        if let Some(some) = self {
            b" ".serialize(writer)?;
            some.serialize(writer)?;
        }

        Ok(())
    }
}
