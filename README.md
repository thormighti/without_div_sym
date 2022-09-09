
     contains the geneic function  which takes generic type of 2 numbers and returns their resut
     when divided
    
     # Example
    
    
     ```
     let a  = 7;
     let b = 2;
    
     assert_eq!(Some(3), without_div_sym::divide(a,b));
     ```
        
    
     # Problems
    
     This crate doesnt accept floating point number yet. i will work on that on next versions.
     the example below will cause an error
    
     ```
     let a = 15.0;
     let b = 5.0;
     assert_eq!(some(3.0), without_div_sym::divide(a,b)); // This wont complie, it will bring out an error
    
    ```
    