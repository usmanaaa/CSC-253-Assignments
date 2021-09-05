use box_vector_1::BoxVec;

fn main() {
    let mut box_vec = BoxVec::new();
    println!("init: {:?}", box_vec);
    box_vec.replace(2, 4);
    println!("replace(2, 4): {:?}", box_vec);
    println!("peek(2): {}", box_vec.peek(2));
    box_vec.remove(2);
    println!("remove(2): {:?}", box_vec);
}
