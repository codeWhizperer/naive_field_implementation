pub mod FieldElement {

    #[derive(Debug, PartialEq)]
    pub struct Field {
        num: i64,
        prime: i64,
    }

    impl Field {
        pub fn new(num: i64, prime: i64) -> Result<Self, String> {
            if num >= prime || num < 0 {
                let error = format!("Num {} not in field range 0 to {}", num, prime - 1);
                return Err(error);
            }
            Ok(Field { num, prime })
        }

        pub fn print_field(&self) {
            println!("FieldElement {} {}", self.num, self.prime);
        }

        pub fn add(&self, other: Field) -> Result<Self, String> {
            if self.prime != other.prime {
                return Err(String::from("Cannot add two numbers in different Fields"));
            }
            let number = (self.num + other.num) % self.prime;
            Ok(Field::new(number, self.prime)?)
        }

        pub fn sub(&self, other: Field) -> Result<Self, String> {
            if self.prime != other.prime {
                return Err(String::from("Cannot sub two numbers in different Fields"));
            }
            let number = (self.num - other.num) % self.prime;
            Ok(Field::new(number, self.prime)?)
        }

        pub fn mul(&self, other: Field) -> Result<Self, String> {
            if self.prime != other.prime {
                return Err(String::from("Cannot mul two numbers in different Fields"));
            }
            let number = (self.num * other.num) % self.prime;
            Ok(Field::new(number, self.prime)?)
        }

        pub fn exp(&self, other: Field) -> Result<Self, String> {
            if self.prime != other.prime {
                return Err(String::from("Cannot exp two numbers in different Fields"));
            }
            let number = (self.num.pow(other.num as u32)) % self.prime;
            Ok(Field::new(number, self.prime)?)
        }
    }

    impl std::fmt::Display for Field {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FieldElement_{}({})", self.prime, self.num)
        }
    }
}
