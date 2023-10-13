mod field;
// fn main() {
//     let field_num = field::FieldElement::Field::new( 7, 13);
// match field_num {
//     Ok(field)=>{
//         println!("Field: {:?}", field)
//     }
//     Err(error) =>{
//         println!("Error: {}", error)
//     }
// }

// }
fn main() {
    let field1 = match field::FieldElement::Field::new(7, 13) {
        Ok(field) => field,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    field1.print_field();

    let field2 = match field::FieldElement::Field::new(4, 6) {
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
}