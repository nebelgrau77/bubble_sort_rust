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


fn main () {
    
    // examples

    let mut arr = [2,5,3,7,1,4,5,6,7,6];
    
    let sorted_arr = bubblesort(&mut arr);

    println!("\nexample 1: integers");

    for item in sorted_arr.iter() {
        print!("{}\t", item);
    }

    let mut arr = [0.01, 0.002, 0.3, 0.011, 0.2, -0.01, 0.01];
    
    let sorted_arr = bubblesort(&mut arr);

    println!("\n\nexample 2: floats");

    for item in sorted_arr.iter() {
        print!("{}\t", item);
    }
    
    let mut arr = ['x', 'a', 'b', 'x', 'p'];
    
    let sorted_arr = bubblesort(&mut arr);

    println!("\n\nexample 3: chars");

    for item in sorted_arr.iter() {
        print!("{}\t", item);
    }

    let mut arr = ["cat", "bat", "rat", "hat"];
    
    let sorted_arr = bubblesort(&mut arr);

    println!("\n\nexample 4: strings");

    for item in sorted_arr.iter() {
        print!("{}\t", item);
    }
    
    print!("\n");

}


