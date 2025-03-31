pub fn get_slice<'a>(buffer: &'a [u8], offset: &mut usize, size: usize) -> &'a [u8] {
	let start_index = *offset;
	let end_index = start_index + size;

	*offset += size;
	&buffer[start_index..end_index]
}
