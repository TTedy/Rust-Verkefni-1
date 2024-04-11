// arrays.rs
// here i will add bubblesort and binary search to find what i need.

fn bubble_sort() {

}

fn binary_search(arr: &mut [i32]) {
    let L = 0;
    let R = (arr.len() - 1);
    let T = arr.iter().max();

    While L <= R{
        let M = ((L + R).floor() / 2);
        let check = if arr[M] < T;{
            L = M - 1;
        } else if arr[m] > T {
            R = M -1;
        }else {
            return M;
        }
        let even = arr[L] = T {
            return L;
        }
      return "unseccessful"
    }
}

fn main() {
    let mut arr: [i32, 5] = [1,6,2,7,8];


    binary_search(&mut arr);




    println!("Hello, world!");
}
