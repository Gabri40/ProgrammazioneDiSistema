fn main() {
    // let isbn=String::from("3-598-21508-8");
    let isbn=String::from("3-598-21507-X");

    let isbn_nodashes=remove_dashes(isbn);

    println!("\n{}",isbn_nodashes);

    println!("\n{}",validate_isbn(isbn_nodashes));
}

fn remove_dashes(isbn:String)->String{
    isbn.replace("-", "")
}

fn validate_isbn(isbn:String)-> i32{

    let mut digit;
    let mut power=10;
    let mut sum=0;
    const RADIX:u32=10;

    for c in isbn.chars(){
        // check if last digit in the isbn code is an X, which corrisponds to value 10 the returns
        if power==1 && c.to_string()=="X" {
            sum+=10;
            return sum%11;
        }
        
        //convert char to number: to_digit returns an optional so .unwrap()
        digit=c.to_digit(RADIX).unwrap();

        // as i32 because both power and digit are u32
        sum+= (digit*power) as i32;
        power-=1;
    }

    return sum%11;
}
