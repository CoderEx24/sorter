
pub fn count_sort(arr: &Vec<u32>, radix: u32, digit_mask: u32) -> Vec<u32> {
    let mut sorted_arr = Vec::with_capacity(arr.len());
    let mut count = Vec::with_capacity(radix as usize);

    for i in 0 .. radix {
        count.push(0);
    }

    for i in 0 .. arr.len() {
        sorted_arr.push(0);
    }
    println!("sorted_arr.len = {}", sorted_arr.len());

    // count
    for val in arr {
        count[ ((val / digit_mask) % radix) as usize ] += 1;
    }

    for i in 1 .. radix {
        count[i as usize] += count[(i - 1) as usize];
    }

    for i in 0 .. arr.len() {
        let idx = ((arr[i as usize] / digit_mask) % radix) as usize;
        sorted_arr[ count[ idx ] - 1 ] = arr[i as usize];
        count[ idx ] -= 1;
    }

    sorted_arr

}
