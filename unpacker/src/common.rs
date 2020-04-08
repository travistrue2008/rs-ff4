use byteorder::{ByteOrder, LittleEndian};
use std::vec;
use std::str;

pub fn get_slice<'a>(buffer: &'a [u8], offset: &mut usize, length: usize) -> &'a [u8] {
    let start_index = *offset as usize;
    let end_index = start_index + length;

    *offset += length;
    &buffer[start_index..end_index]
}

pub fn read_u16(buffer: &[u8], offset: &mut usize) -> u16 {
    let slice = get_slice(buffer, offset, 2);

    LittleEndian::read_u16(slice)
}

pub fn read_u32(buffer: &[u8], offset: &mut usize) -> u32 {
    let slice = get_slice(buffer, offset, 4);

    LittleEndian::read_u32(slice)
}

pub fn read_str(buffer: &[u8], offset: &mut usize, length: usize) -> String {
    let start_index = *offset as usize;
    let end_index = start_index + 48;
    let slice = &buffer[start_index..end_index];
    let mut copy_buffer = vec![0u8; length];

    for (i, value) in slice.iter().enumerate() {
        copy_buffer[i] = if *value >= 32 && *value < 127 {
            *value
        } else {
            32
        };
    }

    let result = str::from_utf8(copy_buffer.as_slice()).unwrap().trim();

    *offset += length;
    String::from(result)
}

pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
	<A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);

    a
}
