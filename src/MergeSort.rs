use core::fmt::Debug;
//Merge two ordered lists, ordering result is copied to worker list l2
fn merge(l1: &Vec<i32>, s: usize, m: usize, e: usize, l2: &mut Vec<i32>) {
    let mut ptr1 = s;
    let mut ptr2 = m;

    for i in s..e {
	if (ptr1 < m) && (ptr2 >= e || l1[ptr1] <= l1[ptr2]) {
	    l2[i] = l1[ptr1];
	    ptr1 += 1;
	} else {
	    l2[i] = l1[ptr2];
	    ptr2 += 1;
	}
    }
}
//Copies from l2 to primary list l1 using mapping inside closure
fn merge_copy(l1: &mut Vec<i32>, s:usize, e: usize, l2: &Vec<i32>) {
    (s..e).for_each(|i| l1[i] = l2[i]);
}

//Splits mutable list into two sub-lists, done recusively until only n sub-lists remain where n=number of elements in original list
fn merge_split(l1: &mut Vec<i32>, s: usize, e:usize, l2: &mut Vec<i32>) {
    if e - s > 1 {
	let m: usize = (e + s) / 2;
	merge_split(l1, s, m, l2);
	merge_split(l1, m, e, l2);
	merge(l1, s, m, e, l2);
	merge_copy(l1, s, e, l2);
    }
}

pub fn sort(list: &mut Vec<i32>) {
    let size: usize = list.len();
    let mut worker: Vec<i32> = vec![0; size];
    merge_split(list, 0, size, &mut worker);
}


/*
pub fn merge_sort(list: Vec<usize>) -> Vec<usize> {
    if list.len() > 1 {
	let (l, r) = list.split_at(list.len() / 2);
	//Repeating merging on a hiher level until original caller  
	let sorted_l = merge_sort(l).to_vec();
	let sorted_r = merge_sort(r).to_vec();
	let mut result = Vec::new(); 
	let (mut i, mut j) = (0, 0);
	let mut k = 0;
	while i < sorted_l.len() && j < sorted_r.len() {
	    if sorted_l[i] <= sorted_r[j] {
		result[k] = sorted_l[i].clone();
		i+=1;
	    } else {
		result[k] = sorted_r[j].clone();
		j += 1;
	    }
	    k += 1;
	}
	while i < sorted_l.len() {
	    result[k] = sorted_l[i].clone();
	    k += 1;
	    i += 1;
	}
	while j < sorted_r.len() {
	    result[k] = sorted_r[j].clone();
	    k += 1;
	    j += 1;
	}
	result
    } else {
	list.to_vec()
    }
}
*/

#[cfg(test)]
mod test {
    use super::*;
     #[test]
    fn basics() {
	let mut list = vec![1, 2, 5, 8, 3, 9];
	sort(&mut list);
	assert_eq!(list, vec![1, 2, 3, 5, 8, 9]);
    }
}
