#!/usr/bin/env python3
"""Generate Rust LeetCode solutions #1635-1714."""
from pathlib import Path
from _gen_1635_1714_bodies import BODIES
from _gen_1635_1714_fixes import FIXES

RUST_DIR = Path(__file__).resolve().parent

SQL_BODY = '''pub const NOTE: &str = "SQL problem; omitted in this set.";
fn main() { println!("{}", NOTE.len()); }
#[cfg(test)]
mod tests { use super::NOTE; #[test] fn note_non_empty() { assert!(!NOTE.is_empty()); } }'''

SQL_NUMS = {1635, 1645, 1651, 1661, 1667, 1677, 1683, 1693, 1699, 1709}

FILES = [
    (1635, "1635_hopper_company_queries_i"),
    (1636, "1636_sort_array_by_increasing_frequency"),
    (1637, "1637_widest_vertical_area_between_two_points_containing_no_points"),
    (1638, "1638_count_substrings_that_differ_by_one_character"),
    (1639, "1639_number_of_ways_to_form_a_target_string_given_a_dictionary"),
    (1640, "1640_check_array_formation_through_concatenation"),
    (1641, "1641_count_sorted_vulnerable_squares_in_chessboard"),
    (1642, "1642_furthest_building_you_can_reach"),
    (1643, "1643_kth_smallest_instructions"),
    (1644, "1644_lowest_common_ancestor_of_a_binary_tree_ii"),
    (1645, "1645_hopper_company_queries_ii"),
    (1646, "1646_get_maximum_in_generated_array"),
    (1647, "1647_minimum_deletions_to_make_character_frequencies_unique"),
    (1648, "1648_sell_diminishing_valued_colored_balls"),
    (1649, "1649_create_sorted_array_through_instructions"),
    (1650, "1650_lowest_common_ancestor_of_a_binary_tree_iii"),
    (1651, "1651_hopper_company_queries_iii"),
    (1652, "1652_defuse_the_bomb"),
    (1653, "1653_minimum_deletions_to_make_string_balanced"),
    (1654, "1654_minimum_jumps_to_reach_home"),
    (1655, "1655_distribute_repeating_integers"),
    (1656, "1656_design_an_ordered_stream"),
    (1657, "1657_determine_if_two_strings_are_close"),
    (1658, "1658_minimum_operations_to_reduce_x_to_zero"),
    (1659, "1659_maximize_grid_happiness"),
    (1660, "1660_correct_a_binary_tree"),
    (1661, "1661_average_time_of_process_per_machine"),
    (1662, "1662_check_if_two_string_arrays_are_equivalent"),
    (1663, "1663_smallest_string_with_a_given_numeric_value"),
    (1664, "1664_ways_to_make_a_fair_array"),
    (1665, "1665_minimum_initial_energy_to_finish_tasks"),
    (1666, "1666_change_the_root_of_a_binary_tree"),
    (1667, "1667_fix_names_in_a_table"),
    (1668, "1668_maximum_trailing_zeros_in_a_cornered_path"),
    (1669, "1669_merge_in_between_linked_lists"),
    (1670, "1670_design_front_middle_back_queue"),
    (1671, "1671_minimum_number_of_removals_to_make_mountain_array"),
    (1672, "1672_richest_customer_wealth"),
    (1673, "1673_find_the_most_competitive_subsequence"),
    (1674, "1674_minimum_moves_to_make_subarray_sum_equal"),
    (1675, "1675_minimize_deviation_in_array"),
    (1676, "1676_lowest_common_ancestor_of_a_binary_tree_iv"),
    (1677, "1677_products_price_in_each_store"),
    (1678, "1678_goal_parser_interpretation"),
    (1679, "1679_max_number_of_k_sum_pairs"),
    (1680, "1680_concatenation_of_consecutive_binary_numbers"),
    (1681, "1681_minimum_incompatibility"),
    (1682, "1682_longest_palindromic_subsequence_ii"),
    (1683, "1683_invalid_tweets"),
    (1684, "1684_count_the_number_of_consistent_strings"),
    (1685, "1685_sum_of_absolute_differences_in_a_sorted_array"),
    (1686, "1686_stone_game_vi"),
    (1687, "1687_delivering_boxes_from_storage_to_ports"),
    (1688, "1688_count_of_matches_in_tournament"),
    (1689, "1689_max_distance_between_a_pair_of_values"),
    (1690, "1690_stone_game_vii"),
    (1691, "1691_maximum_height_by_stacking_cuboids"),
    (1692, "1692_count_ways_to_distribute_candies"),
    (1693, "1693_daily_leads_and_partners"),
    (1694, "1694_reformat_phone_number"),
    (1695, "1695_maximum_erasure_value"),
    (1696, "1696_jump_game_vi"),
    (1697, "1697_checking_existence_of_edge_length_limited_paths"),
    (1698, "1698_number_of_distinct_substrings_in_a_string"),
    (1699, "1699_number_of_calls_between_two_persons"),
    (1700, "1700_number_of_students_unable_to_eat_lunch"),
    (1701, "1701_average_waiting_time"),
    (1702, "1702_maximum_binary_string_after_change"),
    (1703, "1703_minimum_adjacent_swaps_for_k_consecutive_ones"),
    (1704, "1704_determine_if_string_halves_are_alike"),
    (1705, "1705_maximum_number_of_eaten_apples"),
    (1706, "1706_where_will_the_ball_fall"),
    (1707, "1707_maximum_xor_with_an_element_from_array"),
    (1708, "1708_largest_subarray_length_k"),
    (1709, "1709_biggest_window_between_visits"),
    (1710, "1710_maximum_units_on_a_truck"),
    (1711, "1711_count_good_meals"),
    (1712, "1712_ways_to_split_array_into_three_subarrays"),
    (1713, "1713_minimum_operations_to_make_a_subsequence"),
    (1714, "1714_maximum_score_from_removing_substrings"),
]


def title_from_slug(slug: str) -> str:
    parts = slug.split("_", 1)
    name = parts[1] if len(parts) > 1 else slug
    return " ".join(w.capitalize() for w in name.split("_"))


def header(num: int, slug: str) -> str:
    return f"/// LeetCode #{num} - {title_from_slug(slug)}\n"


def get_body(num: int) -> str:
    if num in FIXES:
        return FIXES[num]
    if num in SQL_NUMS:
        return SQL_BODY
    if num not in BODIES:
        raise KeyError(f"Missing body for problem {num}")
    return BODIES[num]


def main() -> None:
    created = 0
    for num, slug in FILES:
        body = get_body(num)
        path = RUST_DIR / f"{slug}.rs"
        path.write_text(header(num, slug) + body, encoding="utf-8")
        created += 1
    print(f"Created {created} files in {RUST_DIR}")


if __name__ == "__main__":
    main()
