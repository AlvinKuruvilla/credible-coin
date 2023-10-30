use num_traits::Num;

/// A trait implementing type conversions of primitive integer container types (slices and vectors)
/// to Vec<&[u8]>, allowing them to be used as input to the rs_merkle Sha256 hash function.
///
/// Safety argument:
/// 1. `as_ptr()`: The `self.as_ptr()` method returns a raw pointer to the start of the slice. Since
///    it's an inherent method on slices and doesn't expose raw pointers outside of the function, it
///    is safe to use.
/// 2. Type compatibility: The raw pointer `byte_data` is converted from *const u32 to *const u8 and
///    then used to create a temporary &[u8] slice using `std::slice::from_raw_parts`. The safety of
///    this conversion relies on ensuring that the raw pointer points to valid memory and that the
///    length does not exceed the size of the accessible memory. In this case, since the raw pointer
///    and length are derived from a valid [u32] slice, we can guarantee their safety.
/// 3. Temporary scope: The creation of the temporary slice within the inner block limits the
///    lifetime of the slice to that block. This ensures that the temporary slice doesn't escape and
///    remains valid only within the safe context.
/// 4. Return value: The returned `byte_slice` is a safe reference to slices of bytes (&[u8]) that
///    were obtained from the temporary slice. Since we're collecting these references into a
///    Vec<&[u8]> within the function and not exposing them outside, it guarantees their validity
///    and safety.

pub trait ToHashable {
    /// Convert the implementing object into a vector of byte slices, suitable for hashing.
    ///
    /// This method facilitates the conversion of an implementing object (like a slice or vector of
    /// integer types) into a `Vec<&[u8]>`. This conversion makes the object compatible for use with
    /// hash functions that expect byte slices as input, rs_merkle Sha256 hash function.
    ///
    /// # Returns
    ///
    /// A `Vec<&[u8]>` where each element represents a byte slice view into the original data of the
    /// implementing object.
    ///
    /// # Safety
    ///
    /// The method guarantees safety through the following precautions:
    ///
    /// 1. `as_ptr()`: Utilizes the `as_ptr()` method to fetch a raw pointer to the start of the slice
    ///    without exposing it outside the function.
    /// 2. **Type compatibility**: Converts the raw pointer from the original type (e.g., `*const u32`)
    ///    to `*const u8`, and creates a temporary slice using `std::slice::from_raw_parts`. This ensures
    ///    that the raw pointer points to valid memory and the specified length does not exceed the size
    ///    of accessible memory.
    /// 3. **Temporary scope**: By creating the temporary slice inside a confined block, it guarantees
    ///    that the slice doesn't outlive its context, preserving safety.
    /// 4. **Return value**: The function only returns safe references, encapsulated within a `Vec<&[u8]>`.
    ///    This ensures the validity of these references and confines them within a safe boundary.
    ///
    /// NOTE: To see usage examples, check the tests folder
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]>;
}
/// Convert a generic slice to a "hashable" Vec<&[u8]>
impl<T> ToHashable for [T]
where
    T: Num,
{
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr().cast::<u8>();
        let byte_len = self.len() * std::mem::size_of_val(self);
        // Safety: Stated above.
        return unsafe {
            let byte_slice = std::slice::from_raw_parts(byte_data, byte_len);
            byte_slice
                .chunks(std::mem::size_of::<T>())
                .collect::<Vec<&[u8]>>()
        };
    }
}
/// Convert a generic slice reference to a "hashable" Vec<&[u8]>
impl<T> ToHashable for &[T]
where
    T: Num,
{
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        let byte_data = self.as_ptr().cast::<u8>();
        let byte_len = self.len() * std::mem::size_of_val(*self);
        // Safety: Stated above.
        return unsafe {
            let byte_slice = std::slice::from_raw_parts(byte_data, byte_len);
            byte_slice
                .chunks(std::mem::size_of::<T>())
                .collect::<Vec<&[u8]>>()
        };
    }
}

/// Convert a generic vector to a "hashable" Vec<&[u8]>
impl<T> ToHashable for Vec<T>
where
    T: Num,
{
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        return self.as_slice().to_hashable_vec_slice();
    }
}
/// Convert a generic reference to a vector to a "hashable" Vec<&[u8]>
impl<T> ToHashable for &Vec<T>
where
    T: Num,
{
    fn to_hashable_vec_slice(&self) -> Vec<&[u8]> {
        return self.as_slice().to_hashable_vec_slice();
    }
}
