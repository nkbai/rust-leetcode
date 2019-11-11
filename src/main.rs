#[macro_use]
#[allow(unused)]
extern crate foreign_types;

mod L0001_two_sum;
mod L0002_add_two_numbers;
mod L0009_palindrome_number;
mod L0015_3sum;
mod L0019_remove_nth_node_from_end_of_list;
mod L0021_merge_two_sorted_lists;
mod L0022_generate_parentheses;
mod L0023_merge_k_sorted_lists;
mod L0083_delete_duplicates;
mod L0094_binary_tree_inorder_traversal;
mod L0203_remove_elements;
mod L0225_implement_stack_using_queue;
mod L997_find_the_town_judge;
mod l0006_zigzag_conversion;
mod l0024_swap_nodes_in_pairs;
mod l0025_reverse_nodes_in_k_group;
mod l0027_remove_element;
mod l0047_permutations_2;
mod l005_longest_palindromic_substring;
mod l0071_simplify_path;
mod l0082_remove_duplicates_from_sort;
mod l0086_partition_list;
mod l008_string_to_integer_atoi;
mod l0091_decode_ways;
mod l0092_reverse_link_list_2;
mod l0095_unique_binary_search_trees;
mod l0096_unique_binary_search_tree;
mod l0099_restore_binary_search_tree;
mod l0145_binary_tree_postorder_traversal;
mod l029_divide_two_integers;
mod l0328_odd_even_linked_list;
mod l032_longest_valid_parentheses;
mod l060_permutation_sequence;
mod l098_validate_bst;
mod l1008_constructor_bst_from_preorder_traversal;
mod l100_same_binary_tree;
mod l101_symmetric_binary_tree;
mod l1026_maximum_difference_between_node_and_ancestor;
mod l1028_recover_tree_from_preorder_traversal;
mod l102_binary_tree_level_order_traversal;
mod l103_binary_tree_zigzag_level_order_traversal;
mod l1042_flower_planting_with_no_adjacent;
mod l104_max_depth_of_binary_tree;
mod l105_construct_binary_tree_from_preorder_andinorder;
mod l106_construct_binary_tree_from_inorder_and_postorder;
mod l107_binary_tree_level_order_traversal_ii;
mod l108_convert_sorted_array_to_bst;
mod l109_convert_sorted_list_to_binary;
mod l1104_path_in_zigzag_labeled_binary_tree;
mod l110_balanced_bst;
mod l1110_delete_nodes_and_return_forest;
mod l111_depth_of_binary_tree;
mod l1123_lowest_common_ancestor_of_deepest_leaves;
mod l112_path_sum;
mod l1130_minium_cost_tree_from_leaf_values;
mod l113_path_sum2;
mod l1145_binary_tree_coloring_game;
mod l114_flattern_binary_tree_to_linked_list;
mod l120_triangle;
mod l124_binary_tree_maximum_path_sum;
mod l129_sum_root_to_leaf_numbers;
mod l144_binary_tree_preorder_traversal;
mod l146_lru_cach3;
mod l146_lru_cache;
mod l146_lru_cache2;
mod l173_binary_search_tree_iterator;
mod l199_binary_tree_right_side_view;
mod l206_reverse_linked_list;
mod l222_count_complete_tree_nodes;
mod l226_invert_binary_tree;
mod l230_kth_smallest_element_in_a_bst;
mod l257_binary_tree_paths;
mod l297_serialize_deserialize_binary_tree;
mod l337_house_robber_3l;
mod l380_insert_delete_getrandom_o1;
mod l402_remove_k_digits;
mod l513_find_bottom_left_tree_value;
mod l515_find_largest_value_in_each_tree;
mod l530_minimum_absolute_difference_in_bst;
mod l538_convert_bst_to_greater_tree;
mod l538_convert_bst_to_greater_tree2;
mod l543_diameter_of_binary_tree;
mod l563_binary_tree_tilt;
mod l572_subtree_of_another_tree;
mod l622_design_circular_queue;
mod l623_add_one_row_to_tree;
mod l641_design_circular_deque;
mod l652_find_duplicate_subtrees;
mod l654_maximum_binary_tree;
mod l655_print_binary_tree;
mod l662_maximum_width_of_binary_tree;
mod l701_insert_into_a_bst;
mod l704_binary_search;
mod l713_subarray_product_less_than_k;
mod l814_binary_tree_pruning;
mod l834_sum_of_distances_in_tree;
mod l863_all_nodes_distance_k_in_binary_tree;
mod l865_smallest_subtree_with_all_the_deepest_nodes;
mod l872_leaf_similiar_trees;
mod l889_construct_binary_tree_from_preorder_and_postorder_traversal;
mod l919_complete_binary_tree_inserter;
mod l925_long_pressed_name;
mod l958_check_completeness_of_a_binary_tree;
mod l987_vectical_order_traversal_of_binary_tree;
mod queue;
mod rust_syntax;
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
    //    testcow();
    main2();
}

//refcell?

use std::cell::RefCell;
fn testRefCell() {
    let x = RefCell::new(vec![1, 2, 3]);
    //    let x2 = x.borrow(); //这种形式的borrow就会崩溃,看来宏的作用于可能是一个问题
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

use std::cell::Cell;

fn main2() {
    let data: Cell<i32> = Cell::new(100);
    let p = &data;
    data.set(10);

    println!("{}", p.get());

    p.set(20);
    println!("{:?}", data);

    let mut data2: Cell<i32> = Cell::new(300);
    let p2 = &mut data2;
    let p3 = p2.get_mut();
    //    let p4 = p2.get_mut();
    *p3 = 40; //因为NLL的缘故,下面一行认为p3生命周期到此结束,所以才有后面的读是可以的.
    println!("p2={}", p2.get());
    //    println!("p3={}", *p3);
}
