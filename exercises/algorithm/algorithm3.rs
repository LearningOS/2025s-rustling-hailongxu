/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM DONE

fn sort<T:PartialOrd+Clone>(array: &mut [T]){
    #[inline]
    fn pppost(i:&mut usize)->usize {
        let t = *i;
        *i += 1; t
    }

    let len = array.len();
    if len <= 1 {
        return;
    }
    let mid = len/2;
    sort(&mut array[..mid]);
    sort(&mut array[mid..]);
    let mut arg = Vec::with_capacity(array.len());
    let (mut i,mut j) = (0,mid);
    while i<mid || j<len {
        let min = if i>=mid {
            pppost(&mut j)
        } else if j>=len {
            pppost(&mut i)
        } else if array[i] <= array[j] {
            pppost(&mut i)
        } else {
            pppost(&mut j)
        };
        arg.push(min);
    }
    let mut t = Vec::with_capacity(len);
    for i in arg {
        t.push(array[i].clone());
    }
    
    array.clone_from_slice(&t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}