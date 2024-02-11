fn main() {
    // working with tuples
    // A tuple is a primitive compound data structure. It allows to group together a number of values with a variety of types into one compound type.

    let tup: (i32, u8, f32) = (34i32, 1u8, 6.0f32);
    // tuple destructuring
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    // using the dot notation to access the tuple values
    println!("The first value is: {}", tup.0);
    println!("The last value is: {}", tup.2);
    // A tuple without any vaules has a special name, unit.
    let _emp_tup: () = ();

    
    // working with arrays
    // let arr = ["Jan", "Feb"];

    // create a tuple to array types
    let t = ([1; 2], [3; 4]);
    println!("{}", t.0[0]);

    // working with functions

}