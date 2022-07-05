pub mod print_vector {

    pub fn print_vector(vec: &Vec <String>){
        for word in vec.iter(){
            println!("{}", word);
        }
    }
}