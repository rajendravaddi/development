fn main(){
    let range = 100;
    for i in 2..range {
        let mut is_prime = true;
        for j in 2..i {
            if i%j==0{
                is_prime = false;
                break;
            }
        }
        if is_prime{
            print!("{}, ",i);
        }
    }
}