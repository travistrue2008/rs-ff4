pub fn get_slice<'a>(buffer: &'a [u8], offset: &mut usize, size: usize) -> &'a [u8] {
	let start_index = *offset;
	let end_index = start_index + size;

	*offset += size;
	&buffer[start_index..end_index]
}

pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
	<A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);

    a
}
