pub mod FieldElement {

    #[derive(Debug, PartialEq)]
    pub struct Field {
        num: i64,
        prime: i64,
    }

    impl Field {
        pub fn new ( num: i64,  prime: i64) ->Result<Self,String>{
            if num >= prime || num < 0 {
                let error = format!("Num {} not in field range 0 to {}", num, prime - 1);
                return Err(error)
            }
           Ok(Field{num, prime})
        }

    pub fn print_field(&self){
        println!("FieldElement {} {}", self.num, self.prime);
    }



    }

    impl std::fmt::Display for Field {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FieldElement_{}({})", self.prime, self.num)
        }
    }
}
