#[test]
fn match_arms_to_easily()
{
 use to_unit::ToUnit; // <-- here

 use std::collections::HashMap;
 let mut x = HashMap::<String, i32>::new();
 x.insert("neko".to_string(), 123);
 match x.get_mut("neko")
 {
  Some(v) => *v = 222,
  None => x.insert("neko".to_string(), 222).to_unit() // <-- here!
 }
 // Ofcorse alternatively, you can write:
 //  eg. None => { x.insert("neko".to_string(), 222); }
 // But, it might be fix to a multi-line format by rust-fmt then...:
 //  eg. None => {
 //          x.insert("neko".to_string(), 222);
 //      }
 // I don't like the multi-line behaviors, so I made the __ lib.
 //
}

/// Or, `use to_unit::__` for ultra lazy persons!
#[test]
fn for_ultra_lazy_persons()
{
 use to_unit::__; // <-- here

 use std::collections::HashMap;
 let mut x = HashMap::<String, i32>::new();
 x.insert("neko".to_string(), 123);
 match x.get_mut("neko")
 {
  Some(v) => *v = 222,
  None => x.insert("neko".to_string(), 222).__() // <-- here!
 }
}
