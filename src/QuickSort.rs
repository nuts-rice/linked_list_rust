fn partition(slice: &mut[i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
	if slice[j] <= pivot {
	    slice.swap(i, j);
	    i += 1
	}
	j += 1;
    }
    slice.swap(i, len - 1);
    i
}

fn quick_sort(slice: &mut [i32]) {
    if !slice.is_empty() {
	let partition_index = partition(slice);
	let len = slice.len();

	quick_sort(&mut slice[0..partition_index]);
	quick_sort(&mut slice[partition_index + 1..len]);
	assert_sorted(slice);
    }
}

fn assert_sorted(slice: &[i32]) {
    for i in 1..slice.len() {
	assert!(slice [i - 1] <= slice[i])
    }
}

#[cfg(test)]
mod test {
    use super::*; 
    #[test]
        fn quicksort_test() {
	let mut list = [1, 5, 3, 8, 4, 9];
	quick_sort(&mut list);
	assert_eq!(list, [1, 3, 4, 5, 8, 9]);
    }
}



