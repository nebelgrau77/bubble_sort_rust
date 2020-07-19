fn main () {
    
    // example 1 

    let mut arr1 = [2,5,3,7,1,4,5,6,7,6];
    
    let sorted_arr1 = bubblesort(&mut arr1);

    for item in sorted_arr1.iter() {
        println!("{}", item);
    }

    // example 2

    let mut arr2 = ["cat", "bat", "rat", "hat"];
    
    let sorted_arr2 = bubblesort(&mut arr2);

    for item in sorted_arr2.iter() {
        println!("{}", item);
    }
    
}


fn bubblesort<T: std::marker::Copy + std::cmp::PartialOrd>(arr: &mut [T]) -> &[T] {
    
    // bubble sort algorithm for generic types that implement Copy and PartialOrd

    let mut temp: T;

    for i in 0..arr.len() {
        
        for j in i..arr.len() {
            
            if arr[i] > arr[j] {                
                temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;                
            }        
        }        
    } 
    arr
}