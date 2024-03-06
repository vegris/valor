use std::collections::HashMap;
use std::hash::Hash;
use std::mem::MaybeUninit;

pub fn get_many_mut<K: Eq + Hash, V, const N: usize>(
    map: &mut HashMap<K, V>,
    keys: [K; N],
) -> Option<[&mut V; N]> {
    for i in 1..N {
        for j in 1..N {
            if i != j && keys[i] == keys[j] {
                return None;
            }
        }
    }

    let mut arr: MaybeUninit<[&mut V; N]> = MaybeUninit::uninit();
    let arr_ptr = arr.as_mut_ptr();

    // SAFETY: We expect `keys` to contain disjunct values that are in bounds of `self`.
    unsafe {
        for (i, key) in keys.iter().enumerate() {
            if let Some(value) = map.get_mut(key) {
                *(*arr_ptr).get_unchecked_mut(i) = &mut *(value as *mut _);
            } else {
                return None;
            }
        }

        Some(arr.assume_init())
    }
}
