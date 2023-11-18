use std::{
    io::{BufReader, BufWriter, Read},
    net::{SocketAddr, TcpStream},
    thread::spawn,
};

use nom::{IResult, Needed};

use libftp::serializer::Serializer;
use libftp::{
    parser::{parse_command, parse_reply},
    serializer::CommandSerializer,
    serializer::ReplySerializer,
};

const BUFFER_SIZE: usize = 1024 * 16;

pub fn proxy_connection(downstream: TcpStream, target: &SocketAddr) -> std::io::Result<()> {
    let upstream = TcpStream::connect(target)?;

    let (downstream_reader, downstream_writer) = tcp_stream_pair(downstream)?;
    let (upstream_reader, upstream_writer) = tcp_stream_pair(upstream)?;

    // Pipe from downstream to upstream and vice versa.
    let forward = spawn(move || {
        pipe(
            downstream_reader,
            parse_command,
            |command| command,
            CommandSerializer::new(upstream_writer),
        )
    });
    let backward = spawn(move || {
        pipe(
            upstream_reader,
            parse_reply,
            |reply| reply,
            ReplySerializer::new(downstream_writer),
        )
    });

    forward.join().unwrap()?;
    backward.join().unwrap()?;

    Ok(())
}

fn tcp_stream_pair(
    stream: TcpStream,
) -> std::io::Result<(BufReader<TcpStream>, BufWriter<TcpStream>)> {
    let cloned = stream.try_clone()?;
    Ok((BufReader::new(stream), BufWriter::new(cloned)))
}

fn pipe<O, R, P, H, S>(mut reader: R, parser: P, hook: H, mut serializer: S) -> std::io::Result<()>
where
    R: Read,
    P: Fn(&[u8]) -> IResult<&[u8], O>,
    H: Fn(O) -> O,
    S: Serializer<O>,
{
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut buffer_index = 0;
    loop {
        break match parse_stream(&mut buffer, &mut buffer_index, &mut reader, &parser)
            .and_then(|parsed| Ok(hook(parsed)))
            .and_then(|parsed| serializer.serialize(&parsed))
        {
            Ok(()) => continue,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(()),
            Err(e) => Err(e),
        }
    }
}

fn parse_stream<O, R, P>(
    buffer: &mut [u8],
    buffer_index: &mut usize,
    reader: &mut R,
    parser: &P,
) -> std::io::Result<O>
where
    R: Read,
    P: Fn(&[u8]) -> IResult<&[u8], O>,
{
    loop {
        let read_buffer = &buffer[..*buffer_index];
        let parse_result = parser(read_buffer);
        match parse_result {
            Ok((unparsed, parsed)) => {
                // Reset buffer.
                let unparsed_amount = unparsed.len();
                if unparsed_amount > 0 {
                    // Index of the unparsed data slice inside the buffer.
                    let unparsed_index = (unparsed.as_ptr() as usize) - (buffer.as_ptr() as usize);

                    // Get a range that represents the unparsed data slice inside the buffer.
                    let unparsed_range = unparsed_index..unparsed_index + unparsed_amount;

                    // Reset buffer with unparsed data.
                    buffer.copy_within(unparsed_range, 0);
                    *buffer_index = unparsed_amount;
                } else {
                    // Reset buffer back to the beginning.
                    *buffer_index = 0;
                }

                return Ok(parsed);
            }
            Err(nom::Err::Incomplete(Needed::Size(amount))) => {
                let amount = amount.get();
                reader.read_exact(&mut buffer[*buffer_index..(*buffer_index + amount)])?;
                *buffer_index += amount;
            }
            Err(nom::Err::Incomplete(Needed::Unknown)) => {
                let read_amount = reader.read(&mut buffer[*buffer_index..])?;
                if read_amount == 0 {
                    return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
                }

                *buffer_index += read_amount;
            }
            Err(error) => {
                dbg!(error);
                panic!();
            }
        };
    }
}
