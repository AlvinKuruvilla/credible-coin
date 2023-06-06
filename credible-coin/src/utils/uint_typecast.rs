/// A trait implementing type conversions of primitive integer container types (slices and vectors)
/// to Vec<&[u8]>, allowing them to be used as input to the sha hash crate

/// Safety argument:

/// 1. as_ptr(): The self.as_ptr() method returns a raw pointer to the start of the slice. Since it's an inherent method on slices and doesn't expose raw pointers outside of the function, it is safe to use.
/// 2. Type compatibility: The raw pointer byte_data is converted from *const u32 to *const u8 and then used to create a temporary &[u8] slice using std::slice::from_raw_parts. The safety of this conversion relies on ensuring that the raw pointer points to valid memory and that the length does not exceed the size of the accessible memory. In this case, since the raw pointer and length are derived from a valid [u32] slice, we can guarantee their safety.
/// 3. Temporary scope: The creation of the temporary slice within the inner block limits the lifetime of the slice to that block. This ensures that the temporary slice doesn't escape and remains valid only within the safe context.
/// 4. Return value: The returned byte_slice is a safe reference to slices of bytes (&[u8]) that were obtained from the temporary slice. Since we're collecting these references into a Vec<&[u8]> within the function and not exposing them outside, it guarantees their validity and safety.

pub trait ToHashable {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]>;
}
/// Convert a u32 slice to a sized Vec<&[u8]>
impl ToHashable for [u32] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u32 slice reference to a sized Vec<&[u8]>
impl ToHashable for &[u32] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}

/// Convert a u64 slice to a sized Vec<&[u8]>
impl ToHashable for [u64] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u64 slice reference to a sized Vec<&[u8]>
impl ToHashable for &[u64] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u16 slice reference to a sized Vec<&[u8]>
impl ToHashable for &[u16] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u16 slice to a sized Vec<&[u8]>
impl ToHashable for [u16] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u128 slice reference to a sized Vec<&[u8]>
impl ToHashable for &[u128] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u128 slice to a sized Vec<&[u8]>
impl ToHashable for [u128] {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u32 vector reference to a sized Vec<&[u8]>
impl ToHashable for &Vec<u32> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u32 vector to a sized Vec<&[u8]>
impl ToHashable for Vec<u32> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a reference to a u64 vector to a sized Vec<> slice of u8
impl ToHashable for &Vec<u64> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u64 vector to a sized Vec<&[u8]>
impl ToHashable for Vec<u64> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a reference to a u16 vector to a sized Vec<> slice of u8
impl ToHashable for &Vec<u16> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u16 vector to a sized Vec<&[u8]>
impl ToHashable for Vec<u16> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}

/// Convert a reference to a u128 vector to a sized Vec<> slice of u8
impl ToHashable for &Vec<u128> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
/// Convert a u128 vector to a sized Vec<&[u8]>
impl ToHashable for Vec<u128> {
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr() as *const u8;
        let byte_len = self.len() * std::mem::size_of::<u32>();

        let byte_slice: &[u8] = {
            let slice = unsafe { std::slice::from_raw_parts(byte_data, byte_len) };

            slice
        };

        byte_slice.chunks(4).collect::<Vec<&[u8]>>()
    }
}
