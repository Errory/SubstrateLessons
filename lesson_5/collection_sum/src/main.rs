fn sum_integer_collections(t: &[u32]) -> Option<u32> {
    if t.len() > 0 {
        let mut total:u32 = 0;
        for item in t.iter() {
            match total.checked_add(*item) {
                Some(s) => total = s,
                None => return None,
            }
        }
        Some(total)
    }
    else {
        None
    }
}

fn main() {
    let arr = vec![4, 7, 9, 11, 17];

    println!("The array sum is {:?}", sum_integer_collections(&arr));
}
