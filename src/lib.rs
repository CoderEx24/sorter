
pub fn count_sort(arr: &[u64], radix: u64, digit_mask: u64) -> Vec<u64> {
    let mut sorted_arr = vec![0; arr.len()]; 
    let mut count = vec![0; radix as usize]; 
    
    // count
    for val in arr {
        count[ ((val / digit_mask) % radix) as usize ] += 1;
    }

    for i in 1 .. radix {
        count[i as usize] += count[(i - 1) as usize];
    }

    for i in (0 .. arr.len()).rev() {
        let idx = ((arr[i as usize] / digit_mask) % radix) as usize;
        sorted_arr[ count[ idx ] - 1 ] = arr[i as usize];
        count[ idx ] -= 1;
    }

    sorted_arr

}

pub fn radix_sort(arr: &Vec<u32>, radix: u32) -> Vec<u32> {
    let mut sorted_arr = Vec::clone(arr);
    let mut digit_mask = 1;
    let max = arr.into_iter().max().unwrap();

    while max / digit_mask > 0 {
        sorted_arr = count_sort(&sorted_arr, radix, digit_mask);
        digit_mask *= radix;

    }

    sorted_arr

}

