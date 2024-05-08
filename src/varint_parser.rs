pub fn varint_parser(page: &Vec<u8>, pointer: &mut usize) -> usize {
  // Basically read data until the
  // MSB is 0, meaning that no more data will follow
  let mask = 0b01111111;
  let mut result = 0;
  loop {
      let current_value = page[*pointer] & mask;
      result = (result << 7) | current_value as usize;
      // literally looking for something like 0b00000000
      if (page[*pointer] >> 7) & 1 == 0 {
          break;
      }
      *pointer += 1;
  }
  *pointer += 1;
  result
}