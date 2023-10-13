use field_element::field_impl::field_impl::FieldElement::Field;
 

fn main() {
    let field1 = match Field::new(44, 57) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    field1.print_field();

    let field2 = match Field::new(33, 57) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    // Check for equality
    if field1 == field2 {
        println!("Fields are equal");
    } else {
        println!("Fields are not equal");
    }

    // addition implementation of field:

    match field1.add(field2) {
        Ok(result) => println!("Addition result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    let field3 = match Field::new(9, 57) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let field4 = match Field::new(29, 57) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    match field3.sub(field4) {
        Ok(result) => println!("Subtraction result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    // mutiplication of field:

    let mul_one = match Field::new(3, 13) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let mul_two = match Field::new(12, 13) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    match mul_one.mul(mul_two) {
        Ok(result) => println!("Multiplication result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    ///////////////pow

    let exp_one = match Field::new(3, 13) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let exp_two = match Field::new(1, 13) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    match exp_one.exp(exp_two) {
        Ok(result) => println!("Exponential result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }
}
