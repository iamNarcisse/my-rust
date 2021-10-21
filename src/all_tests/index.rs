use core::num;

pub fn add_numbers(a:i32, b:i32) -> i32 {
   let result : i32 =  a + b;

   print!("Result is {:?}", result);

   result
}


pub fn bad_add_number(a : i32, b : i32)  -> i32{
    a - b
}


pub fn sqrt(number : f64) ->  Result<f64, String> {

    if number >= 0.0{
     return   Ok(number.powf(0.5))
    }

   Err("negative floats don't have square roots".to_owned())

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


    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
    

    

}