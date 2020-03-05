#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
extern crate foreign_types;

mod a1_template;
mod interview;
mod l0002_add_two_numbers;
mod l0006_zigzag_conversion;
mod l0009_palindrome_number;
mod l0015_3sum;
mod l0019_remove_nth_node_from_end_of_list;
mod l0021_merge_two_sorted_lists;
mod l0022_generate_parentheses;
mod l0023_merge_k_sorted_lists;
mod l0024_swap_nodes_in_pairs;
mod l0025_reverse_nodes_in_k_group;
mod l0027_remove_element;
mod l0047_permutations_2;
mod l005_longest_palindromic_substring;
mod l0071_simplify_path;
mod l0082_remove_duplicates_from_sort;
mod l0083_delete_duplicates;
mod l0086_partition_list;
mod l008_string_to_integer_atoi;
mod l0091_decode_ways;
mod l0092_reverse_link_list_2;
mod l0094_binary_tree_inorder_traversal;
mod l0095_unique_binary_search_trees;
mod l0096_unique_binary_search_tree;
mod l0099_restore_binary_search_tree;
mod l0145_binary_tree_postorder_traversal;
mod l0203_remove_elements;
mod l0225_implement_stack_using_queue;
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
mod l1052_grumpy_bookstore_owner;
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
mod l1143_longest_common_subsequence;
mod l1145_binary_tree_coloring_game;
mod l114_flattern_binary_tree_to_linked_list;
mod l120_triangle;
mod l121_best_time_to_buy_and_sell_stock;
mod l122_best_time_to_buy_and_sell_stock;
mod l123_best_time_to_buy_and_sell_stock_3;
mod l124_binary_tree_maximum_path_sum;
mod l128_longest_consecutive_sequence;
mod l1297_maximum_number_of_occurrences_of_a_substring;
mod l1297_maximum_number_of_occurrences_of_a_substring2;
mod l129_sum_root_to_leaf_numbers;
mod l130_surrounded_regions;
mod l134_gas_station;
mod l135_candy;
mod l143_reorder_list;
mod l144_binary_tree_preorder_traversal;
mod l146_lru_cach3;
mod l146_lru_cache;
mod l146_lru_cache2;
mod l149_max_points_on_a_line;
mod l152_maximum_product_subarray;
mod l155_min_stack;
mod l166_fraction_to_recurring_decimal;
mod l173_binary_search_tree_iterator;
mod l187_repeated_dna_sequences;
mod l188_best_time_to_buy_and_sell_stock_iv;
mod l199_binary_tree_right_side_view;
mod l1_two_sum;
mod l200_numbers_of_islands;
mod l206_reverse_linked_list;
mod l207_course_schedule;
mod l209_minimum_size_subarray_sum;
mod l210_course_schedule2;
mod l215_kth_largest_element_in_an_array;
mod l220_contains_duplicate_iii;
mod l222_count_complete_tree_nodes;
mod l226_invert_binary_tree;
mod l230_kth_smallest_element_in_a_bst;
mod l232_implement_queue_using_stack;
mod l257_binary_tree_paths;
mod l260_single_number_iii;
mod l287_find_the_duplicate_number;
mod l287_find_the_duplicate_number2;
mod l288_unique_word_abbreviation;
mod l290_word_pattern;
mod l297_serialize_deserialize_binary_tree;
mod l309_best_time_to_buy_and_sell_stock_with_cooldown;
mod l30_substring_with_concatenation_of_all_words;
mod l30_substring_with_concatenation_of_all_words_2;
mod l31_next_permutation;
mod l329_longest_increasing_path_in_a_matrix;
mod l337_house_robber_3l;
mod l343_integer_break;
mod l365_water_and_jug_problem;
mod l365_water_and_jug_problem2;
mod l36_valid_sudoku;
mod l380_insert_delete_getrandom_o1;
mod l386_lexicolgraphical_numbers;
mod l389_find_the_difference;
mod l39_combination_sum;
mod l402_remove_k_digits;
mod l409_longest_palindrome;
mod l42_traping_rain_water;
mod l42_traping_rain_water_2;
mod l438_find_all_anagrams_in_a_string;
mod l452_mininum_number_of_arrows_to_burst_ballons;
mod l454_sum_2;
mod l463_island_perimeter;
mod l476_number_complement;
mod l488_zuma_game;
mod l494_target_sum;
mod l49_group_anagrams;
mod l4_median_of_two_sorted_arrays;
mod l513_find_bottom_left_tree_value;
mod l515_find_largest_value_in_each_tree;
mod l51_n_queens;
mod l523_continuous_subarray_sum;
mod l525_contiguous_array;
mod l530_minimum_absolute_difference_in_bst;
mod l535_encode_and_decode_tinyurl;
mod l538_convert_bst_to_greater_tree;
mod l538_convert_bst_to_greater_tree2;
mod l53_maxinum_subarray;
mod l543_diameter_of_binary_tree;
mod l547_friend_circles;
mod l554_brick_wall;
mod l55_jump_game;
mod l55_jump_game_2;
mod l560_subarray_sum_equals_k;
mod l560_subarray_sum_equals_k_2;
mod l563_binary_tree_tilt;
mod l572_subtree_of_another_tree;
mod l622_design_circular_queue;
mod l623_add_one_row_to_tree;
mod l632_smallest_range_covering_elements_from_k_lists;
mod l641_design_circular_deque;
mod l64_minimum_path_sum;
mod l652_find_duplicate_subtrees;
mod l654_maximum_binary_tree;
mod l655_print_binary_tree;
mod l662_maximum_width_of_binary_tree;
mod l673_number_of_longest_increasing_subsequence;
mod l684_redundant_connection;
mod l684_redundant_connection2;
mod l685_redundant_connection_2;
mod l687_longest_univalue_path;
mod l701_insert_into_a_bst;
mod l704_binary_search;
mod l713_subarray_product_less_than_k;
mod l714_best_time_to_buy_and_sell_stocks_with_fee;
mod l72_edit_distance;
mod l76_mininum_window_substring;
mod l78_subsets;
mod l79_word_search;
mod l80_remove_duplicates_from_sorted_array_2;
mod l814_binary_tree_pruning;
mod l834_sum_of_distances_in_tree;
mod l84_largest_rectangle_in_histogram;
mod l84_largest_rectangle_in_histogram_2;
mod l85_maximal_rectangle;
mod l863_all_nodes_distance_k_in_binary_tree;
mod l865_smallest_subtree_with_all_the_deepest_nodes;
mod l872_leaf_similiar_trees;
mod l889_construct_binary_tree_from_preorder_and_postorder_traversal;
mod l897_increasing_order_search_tree;
mod l919_complete_binary_tree_inserter;
mod l925_long_pressed_name;
mod l958_check_completeness_of_a_binary_tree;
mod l97_interleaving_string;
mod l97_interleaving_string_2;
mod l987_vectical_order_traversal_of_binary_tree;
mod l997_find_the_town_judge;
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
#[allow(dead_code)]
fn test_ref_cell() {
    let x = RefCell::new(vec![1, 2, 3]);
    //    let x2 = x.borrow(); //这种形式的borrow就会崩溃,看来宏的作用于可能是一个问题
    println!("{:?} readborrow", x.borrow());
    x.borrow_mut().push(4);
    println!("{:?} write borrow", x.borrow_mut());
}

use std::borrow::Cow;
#[allow(dead_code)]
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
#[allow(dead_code)]
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
