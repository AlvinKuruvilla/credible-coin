//TODO: I think a better overall design ould be to have 2 traits
// that implement these functions (and the other endians) for slice 
// and Vec parameters respectiveely

/// Convert a refrence to a u32 slice to a sized Vec<> slice of u8
pub fn u32_slice_to_byte_vector(slice: &[u32]) -> Vec<[u8; 4]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}
/// Convert a refrence to a u64 slice to a sized Vec<> slice of u8
pub fn u64_slice_to_byte_vector(slice: &[u64]) -> Vec<[u8; 8]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}
/// Convert a refrence to a u16 slice to a sized Vec<> slice of u8
pub fn u16_slice_to_byte_vector(slice: &[u16]) -> Vec<[u8; 2]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}
/// Convert a refrence to a u128 slice to a sized Vec<> slice of u8
pub fn u128_slice_to_byte_vector(slice: &[u128]) -> Vec<[u8; 16]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}
/// Convert a refrence to a u32 vector to a sized Vec<> slice of u8
pub fn u32_vector_to_byte_vector(slice: &Vec<u32>) -> Vec<[u8; 4]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}
/// Convert a refrence to a u64 vector to a sized Vec<> slice of u8
pub fn u64_vector_to_byte_vector(slice: &Vec<u64>) -> Vec<[u8; 8]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}
/// Convert a refrence to a u16 vector to a sized Vec<> slice of u8
pub fn u16_vector_to_byte_vector(slice: &Vec<u16>) -> Vec<[u8; 2]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}
/// Convert a refrence to a u128 vector to a sized Vec<> slice of u8
pub fn u128_vector_to_byte_vector(slice: &Vec<u128>) -> Vec<[u8; 16]> {
    let byte_slice: Vec<_> = slice.iter().map(|x| x.to_ne_bytes()).collect();
    return byte_slice;
}

