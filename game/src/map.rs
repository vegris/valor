use std::collections::HashMap;
use std::hash::Hash;
use std::mem::MaybeUninit;

pub struct Map<K, V>(pub HashMap<K, V>);

impl<K, V> Map<K, V>
where
    K: PartialEq + Eq + Hash,
{
    pub fn get_many_mut<const N: usize>(&mut self, keys: [K; N]) -> Option<[&mut V; N]> {
        for index in 1..N {
            if keys[index] == keys[index - 1] {
                return None;
            }
        }

        let mut arr: MaybeUninit<[&mut V; N]> = MaybeUninit::uninit();
        let arr_ptr = arr.as_mut_ptr();

        // SAFETY: We expect `keys` to contain disjunct values that are in bounds of `self`.
        unsafe {
            for (i, key) in keys.iter().enumerate() {
                if let Some(value) = self.0.get_mut(key) {
                    *(*arr_ptr).get_unchecked_mut(i) = &mut *(value as *mut _);
                } else {
                    return None;
                }
            }

            Some(arr.assume_init())
        }
    }
}
