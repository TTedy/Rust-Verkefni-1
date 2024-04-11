// arrays.rs
// here i will add bubblesort and binary search to find what i need.

fn bubble_sort() {

}

fn binary_search(arr: &[usize],find:usize) -> Option {
    let mut L = 0;
    let mut Length = arr.len();
    let mut H = Length / 2;
    let mut R = Length - 1; 
    let mut current = arr[half];  

    while L <= R {
        if current == find {
            return Some(half)
        }
        else if current < find{ lind = half + 1; } else if current > find {
            R = H -1;
        }
        H = (H + L ) / 2;
        current = arr[H];
    }
    return None;
}

fn main() {
    let mut arr: [i32; 5] = [1,6,2,7,8];


    binary_search(&mut arr);




    println!("Hello, world!");
}
