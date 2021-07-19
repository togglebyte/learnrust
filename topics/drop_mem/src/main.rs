fn move_the_vec(mut a_vec: Vec<u32>) -> Vec<u32>{
    a_vec.push(4);
    a_vec
} 

fn make_and_use_vec() {

    // New scope
    {
        let mut vec = vec![1, 2, 3];
        let vec = move_the_vec(vec); // vec is not available in this scope anymore
        eprintln!("{:?}", vec);
    }  // <-- memory of `vec` is marked as free 

    // eprintln!("{:?}", vc);
} 

fn main() {
    make_and_use_vec();

}
