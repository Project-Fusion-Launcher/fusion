use crate::api::Guid;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    cmp::Ordering,
    io::{Read, Seek, SeekFrom},
};

pub fn read<R, U>(reader: &mut R) -> Result<U>
where
    R: ReadBytesExt + Seek,
    U: Readable,
{
    Ok(U::read(reader)?)
}

pub trait Readable: Sized {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self>;
}

impl Readable for u8 {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u8()
    }
}

impl Readable for u32 {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u32::<LittleEndian>()
    }
}

impl Readable for i32 {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_i32::<LittleEndian>()
    }
}

impl Readable for u64 {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u64::<LittleEndian>()
    }
}

impl Readable for i64 {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_i64::<LittleEndian>()
    }
}

impl Readable for Guid {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        let a = reader.read_u32::<LittleEndian>()?;
        let b = reader.read_u32::<LittleEndian>()?;
        let c = reader.read_u32::<LittleEndian>()?;
        let d = reader.read_u32::<LittleEndian>()?;
        Ok((a, b, c, d))
    }
}

impl Readable for [u8; 4] {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Readable for [u8; 16] {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Readable for [u8; 20] {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 20];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Readable for [u8; 32] {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 32];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Readable for String {
    fn read<R: ReadBytesExt + Seek>(reader: &mut R) -> std::io::Result<Self> {
        read_fstring(reader).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to read string: {}", e),
            )
        })
    }
}

fn read_fstring<R: Read + Seek>(reader: &mut R) -> Result<String> {
    let length: i32 = read(reader)?;

    match length.cmp(&0) {
        Ordering::Less => {
            let byte_length = (-length * 2) as usize;
            let mut buffer = vec![0u8; byte_length - 2];
            reader.read_exact(&mut buffer)?;
            reader.seek(SeekFrom::Current(2))?;

            let utf16: Vec<u16> = buffer
                .chunks(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();

            Ok(String::from_utf16(&utf16)?)
        }
        Ordering::Greater => {
            let mut buffer = vec![0u8; (length - 1) as usize];
            reader.read_exact(&mut buffer)?;
            reader.seek(SeekFrom::Current(1))?;

            Ok(String::from_utf8(buffer)?)
        }
        Ordering::Equal => Ok(String::new()),
    }
}
