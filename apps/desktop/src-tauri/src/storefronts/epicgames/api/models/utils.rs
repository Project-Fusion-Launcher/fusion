use crate::common::result::Result;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn read<T, U>(cursor: &mut T) -> Result<U>
where
    T: ReadBytesExt,
    U: Readable,
{
    Ok(U::read(cursor)?)
}

pub trait Readable: Sized {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self>;
}

impl Readable for u8 {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u8()
    }
}

impl Readable for u32 {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u32::<LittleEndian>()
    }
}

impl Readable for u64 {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u64::<LittleEndian>()
    }
}

impl Readable for [u8; 4] {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Readable for [u8; 16] {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Readable for [u8; 20] {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 20];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Readable for [u8; 32] {
    fn read<R: ReadBytesExt>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 32];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}
