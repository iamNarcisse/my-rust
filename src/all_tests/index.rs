pub fn add_numbers(a:i32, b:i32) -> i32 {
   let result : i32 =  a + b;

   print!("Result is {:?}", result);

   result
}


pub fn bad_add_number(a : i32, b : i32)  -> i32{
    a - b
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_numbers(1, 2), 3);
    }



    #[test]
    fn bad_add() {
       assert_eq!(bad_add_number(1, 2), 3);
    }


}