pub struct Sort {}

impl Sort {
    pub fn new() -> Sort {
        Sort{}
    }

    fn exchange(arr: &mut [i32], from: usize, to: usize) {
        let t = arr[from];
        arr[from] = arr[to];
        arr[to] = t;
    }

    pub fn selectionSort(arr: &mut [i32]) {
        let n = arr.len();
        if n < 1 {
            return;
        }

        for idx in 0..n {
            let mut minIdx = idx;
            for jdx in idx..n {
                if arr[jdx] < arr[minIdx] {
                    minIdx = jdx;
                }
            }
            Self::exchange(arr, idx, minIdx);
        }
    }

    pub fn insertionSort(arr: &mut [i32]) {
        let n = arr.len();
        if n < 1 {
            return;
        }
        
        for i in 1..n {            
            for j in (1..(i+1)).rev() {                    
                if arr[j] < arr[j-1] {
                    Self::exchange(arr, j, j-1);
                } else {
                    break;
                }
            }                 
        }
    }

    pub fn shellSort(arr: &mut [i32]) {
        let n = arr.len();
        if n < 1 {
            return;
        }

        let mut h = 1;
        while h < n/3 {
            h = h * 3 + 1;
        }

        while h >= 1 {
            let mut i = h;
            while i < n {

                let mut j = i;
                while j >= h {
                    if arr[j] < arr[j-h] {
                        Self::exchange(arr, j, j-h);
                    } else {
                        break;
                    }
                    j -= h;
                }

                i += 1;
            }

            h = h / 3;
        }
    }

    fn _merge(arr: &mut [i32], lo: usize, mid: usize, hi: usize) {        
        let mut tmp: [i32; 100] = [0; 100]; // tmp size 100

        for idx in lo..(hi+1) {
            tmp[idx] = arr[idx];
        }

        let mut i = lo;
        let mut j = mid + 1;

        for k in lo..(hi+1) {
            if i > mid {
                arr[k] = tmp[j];
                j += 1;
            } else if j > hi {
                arr[k] = tmp[i];
                i += 1;
            } else if arr[j] < arr[i] {
                arr[k] = tmp[j];
                j += 1;
            } else {
                arr[k] = tmp[i];
                i += 1;
            }
        }
    }

    fn _mergeSort(arr: &mut [i32], lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }

        let mid = (lo + hi)/2;
        Self::_mergeSort(arr, lo, mid);
        Self::_mergeSort(arr, mid+1, hi);

        Self::_merge(arr, lo, mid, hi);
    }

    pub fn mergeSort(arr: &mut [i32]) {
        let n = arr.len();
        if n < 1 {
            return;
        }

        Self::_mergeSort(arr, 0, n-1);
    }

    fn _quickSort(arr: &mut [i32], lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }

        let mut i = lo;
        let mut j = hi + 1;
        let base = arr[lo];

        loop {
            i += 1;
            while arr[i] <= base {
                if i == hi { break; }
                i += 1;
            }

            j -= 1;
            while arr[j] >= base {
                if j == lo { break; }
                j -= 1;
            }

            if i >= j {
                break;
            }

            Self::exchange(arr, i, j);
        }
        Self::exchange(arr, lo, j);

        if j > lo {
            Self::_quickSort(arr, lo, j-1);
        }

        if j < hi {
            Self::_quickSort(arr, j+1, hi);
        }        
    }

    pub fn quickSort(arr: &mut [i32]) {
        let n = arr.len();
        if n < 1 {
            return;
        }

        Self::_quickSort(arr, 0, n-1);
    }
}