mod L0001_two_sum;
mod L0002_add_two_numbers;
mod L0009_palindrome_number;
mod L0015_3sum;
mod L0019_remove_nth_node_from_end_of_list;
mod L0021_merge_two_sorted_lists;
mod L0022_generate_parentheses;
mod L0023_merge_k_sorted_lists;
mod L0083_delete_duplicates;
mod L0203_remove_elements;
mod L0225_implement_stack_using_queue;
mod L997_find_the_town_judge;
mod l0006_zigzag_conversion;
mod l1042_flower_planting_with_no_adjacent;
mod queue;
mod share;
mod shortestpath;
mod stack;
fn test(y: &str) -> &str {
    y
}
fn main() {
    let result;
    {
        //        let string1 = String::from("long string is long");;
        let b = "long string is long"; //string1.as_str();
                                       //        let b=string1.as_str();
        result = test(b);
    }
    println!("string is {}", result);
    testcow();
}

//refcell?

use std::cell::RefCell;
fn testRefCell() {
    let x = RefCell::new(vec![1, 2, 3]);
    let x2 = x.borrow(); //这种形式的borrow就会崩溃,看来宏的作用于可能是一个问题
    println!("{:?} readborrow", x.borrow());
    x.borrow_mut().push(4);
    println!("{:?} write borrow", x.borrow_mut());
}

use std::borrow::Cow;
fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            //to_mut这次调用才是真正的clone,其他时候并不clone
            input.to_mut()[i] = -v;
        }
    }
}

//fn abs_sum(input: &mut Cow<[i32]>) {
//    abs_all(input);
//    input.iter().fold(0, |acc, n| acc + n)
//}
fn testcow() {
    let s1 = [1, -2, 3];
    let mut i1 = Cow::from(&s1[..]);
    let i2 = abs_all(&mut i1);
    println!("i1={:?}", i1);
    println!("i2={:?}", i2);
    //    let s3 = "";
    //    s3.parse();
}
