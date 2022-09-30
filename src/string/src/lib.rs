mod l1154_day_of_year;
mod l1108_defang_ip_addr;
mod l1078_find_ocurrences;
mod l1071_gcd_of_strings;
mod l1047_remove_duplicates;
mod l1021_remove_outer_parentness;
mod l1002_common_characters;
mod l953_alien_dictionary_greater;
mod l944_min_deletion_size;
mod l942_di_string_match;
mod l929_num_unique_emails;
mod l917_reverse_only_letters;
mod l884_uncommon_from_setences;
mod l859_buddy_strings;
mod l844_backspace_string_compare;
mod l830_large_group_positions;
mod l824_goat_latin;
mod l819_most_common_words;
mod l796_rotate_string;
mod l771_jewel_stones;
mod l709_to_lower_case;
mod l696_count_binary_substrings;
mod l680_valid_palindrome_2;
mod l657_robot_return_to_origin;
mod l557_reverse_words_in_string_3;
mod l551_student_attendance_record_1;
mod l521_longest_uncommon_subsequence_1;
mod l14_longest_common_prefix;
mod l12_roman_to_integer;
mod l541_reverse_string_2;
mod l520_detect_capital;
mod l2309_greatest_english_letter_in_upper_and_lower_case;
mod l2315_count_asterisks;
mod l2325_decode_message;
mod l2351_first_letter_to_appear_twice;
mod l2379_min_recolor;
mod l2399_check_distance;

fn strs_to_strings(ss:Vec<&str>) -> Vec<String> {
    ss.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_distance() {
        let s = "abaccb".to_string();
        let dis = [1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
        let res = l2399_check_distance::Solution::check_distances(s, dis);
        assert_eq!(res, true);
    }

    #[test]
    fn minimum_recolors() {
        let s = "WBBWWBBWBW".to_string();
        let res = l2379_min_recolor::Solution::minimum_recolors(s, 7);
        assert_eq!(res, 3);
    }

    #[test]
    fn appear_twice() {
        let s = "abccbaacz".to_string();
        let res = l2351_first_letter_to_appear_twice::Solution::repeated_character(s);
        assert_eq!(res, 'c');
    }
    #[test]
    fn decode_message() {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let msg = "vkbs bs t suepuv".to_string();
        let res = l2325_decode_message::Solution::decode_message(key,msg);
        assert_eq!(res, "this is a secret".to_string());
    }

    #[test]
    fn count_asterisks() {
        let s = "l|*e*et|c**o|*de|".to_string();
        let res = l2315_count_asterisks::Solution::count_asterisks(s);
        assert_eq!(res, 2);
    }

    #[test]
    fn greatest_letter() {
        let s = "lEeTcOdE".to_string();
        let res = l2309_greatest_english_letter_in_upper_and_lower_case::Solution::greatest_letter(s);
        assert_eq!(res, "E".to_string());
    }

    #[test]
    fn detect_capital_use() {
        let s = "China".to_string();
        let res = l520_detect_capital::Solution::detect_capital_use(s);
        assert_eq!(res, true);
    }

    #[test]
    fn reverse_string_2() {
        let s = "abcdefg".to_string();
        let res = l541_reverse_string_2::Solution::reverse_str(s, 2);
        assert_eq!(res, "bacdfeg".to_string());
    }

    #[test]
    fn roman_to_int() {
        let s = "MCMXCIV".to_string();
        let res = l12_roman_to_integer::Solution::roman_to_int(s);
        assert_eq!(res, 1994);
    }

    #[test]
    fn longest_common_prefix() {
        let ss = strs_to_strings(["flower","flow","flight"].to_vec());
        let res = l14_longest_common_prefix::Solution::longest_common_prefix(ss);
        println!("{}", res);
        assert_eq!(res, "fl".to_string());
    }
    #[test]
    fn find_lu_slength() {
        let s = "aba".to_string();
        let b = "abc".to_string();
        let res = l521_longest_uncommon_subsequence_1::Solution::find_lu_slength(s, b);
        assert_eq!(res, 3);
    }

    #[test]
    fn check_records() {
        let s = "PPALLP".to_string();
        let res = l551_student_attendance_record_1::Solution::check_record(s);
        assert_eq!(res, true);
    }
    #[test]
    fn reverse_words_in_string_3() {
        let s = "Let's take LeetCode contest".to_string();
        let want = "s'teL ekat edoCteeL tsetnoc".to_string();
        let res = l557_reverse_words_in_string_3::Solution::reverse_words(s);
        assert_eq!(res, want);
    }

    #[test]
    fn judge_circle() {
        let s = "UDLR".to_string();
        let res = l657_robot_return_to_origin::Solution::judge_circle(s);
        assert_eq!(res, true);
    }
    #[test]
    fn valid_palindrome_2() {
        let s = "abca".to_string();
        let res = l680_valid_palindrome_2::Solution::valid_palindrome(s);
        assert_eq!(res, true);
    }

    #[test]
    fn count_binary_substring() {
        let s = "00110011".to_string();
        let res = l696_count_binary_substrings::Solution::count_binary_substrings(s);
        assert_eq!(res, 6);
    }

    #[test]
    fn to_lower_case() {
        let s = "Hello".to_string();
        let want = "hello".to_string();
        let res = l709_to_lower_case::Solution::to_lower_case(s);
        assert_eq!(res, want);
    }

    #[test]
    fn jewels_and_stones() {
        let j = "aA".to_string();
        let s = "aAAbbbb".to_string();
        let res = l771_jewel_stones::Solution::num_jewels_in_stones(j, s);
        assert_eq!(res, 3);
    }
    #[test]
    fn rotate_string() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        let res = l796_rotate_string::Solution::rotate_string(s, goal);
        assert_eq!(res, true);
    }
    #[test]
    fn vec_string_to_string() {
        let ss = vec!["nihao".to_string(), "hello".to_string()];
        let s = ss.iter().map(|x|x.clone()).collect::<String>();
        let want = "nihaohello".to_string();
        assert_eq!(s, want);
        let s1 = ss.join("");
        assert_eq!(s1, want);
    }

    #[test]
    fn most_common_words() {
        let g = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
        let banned = vec!["hit".to_string()];
        let want = "ball".to_string();
        let res = l819_most_common_words::Solution::most_common_word(g, banned);
        assert_eq!(res, want);
    }
    #[test]
    fn to_goat_latin() {
        let s = "I speak Goat Latin".to_string();
        let res = l824_goat_latin::Solution::to_goat_latin(s);
        let want = "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string();
        assert_eq!(res, want);
    }

    #[test]
    fn strs_to_strings_test() {
        let s = vec!["1", "2"];
        let want = vec!["1".to_string(), "2".to_string()];
        let res = strs_to_strings(s);
        assert_eq!(res, want);
    }

    #[test]
    fn day_of_year() {
        let result = l1154_day_of_year::Solution::day_of_year("2022-09-21".to_string());
        println!("{:?}", result);
        assert_eq!(result, 264);
    }
    #[test]
    fn defang_ip_addr() {
        let s = "1.1.1.1".to_string();
        let want = "1[.]1[.]1[.]1".to_string();
        let res = l1108_defang_ip_addr::Solution::defang_i_paddr(s);
        assert_eq!(res, want);
    }

    #[test]
    fn find_ocurrences() {
        let s = "alice is a good girl she is a good student".to_string();
        let want = vec!["girl".to_string(),"student".to_string()];
        let res = l1078_find_ocurrences::Solution::find_ocurrences(
            s, "a".to_string(), "good".to_string());
        assert_eq!(res, want);
    }

    #[test]
    fn gcd_of_strings() {
        let s1 = "ABABAB".to_string();
        let s2 = "ABAB".to_string();
        let want = "AB".to_string();
        let res = l1071_gcd_of_strings::Solution::gcd_of_strings(s1, s2);
        assert_eq!(res, want);
    }

    #[test]
    fn remove_duplicates() {
        let s = "abbaca".to_string();
        let want = "ca".to_string();
        let res = l1047_remove_duplicates::Solution::remove_duplicates(s);
        assert_eq!(res, want);
    }

    #[test]
    fn remove_outer_parentness() {
        let s = "(()())(())".to_string();
        let want = "()()()".to_string();
        let res = l1021_remove_outer_parentness::Solution::remove_outer_parentheses(s);
        assert_eq!(res, want);
    }

    #[test]
    fn common_chars() {
        let s =  strs_to_strings(vec!["bella","label","roller"]);
        let want = strs_to_strings(vec!["e", "l", "l"]);
        let res = l1002_common_characters::Solution::common_chars(s);
        assert_eq!(res, want);
    }
    #[test]
    fn alien_dictionary() {
        let s = strs_to_strings(vec!["hello","leetcode"]);
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        let res = l953_alien_dictionary_greater::Solution::is_alien_sorted(s, order);
        assert_eq!(res, true);
    }

    #[test]
    fn min_deletion_size() {
        let s = strs_to_strings(vec!["cba","daf","ghi"]);
        let want = 1;
        let res = l944_min_deletion_size::Solution::min_deletion_size(s);
        assert_eq!(res, want);
    }

    #[test]
    fn di_string_match() {
        let s = "IDID".to_string();
        let want = vec![0,4,1,3,2];
        let res = l942_di_string_match::Solution::di_string_match(s);
        assert_eq!(res, want);
    }

    #[test]
    fn num_unique_emails() {
        let s = strs_to_strings(vec!["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]);
        let want = 2;
        let res = l929_num_unique_emails::Solution::num_unique_emails(s);
        assert_eq!(res, want);
    }

    #[test]
    fn reverse_only_letters() {
        let s = "a-bC-dEf-ghIj".to_string();
        let want = "j-Ih-gfE-dCba".to_string();
        let res = l917_reverse_only_letters::Solution::reverse_only_letters(s);
        assert_eq!(res, want);
    }

    #[test]
    fn uncommon_from_sentences() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        let want = strs_to_strings(vec!["sweet","sour"]);
        let res = l884_uncommon_from_setences::Solution::uncommon_from_sentences(s1, s2);
        assert_eq!(res, want);
    }

    #[test]
    fn buddy_strings() {
        let s = "ab".to_string();
        let goal = "ba".to_string();
        let res = l859_buddy_strings::Solution::buddy_strings(s, goal);
        assert_eq!(res, true);
    }

    #[test]
    fn backspace_string_compare() {
        let s1 = "ab##".to_string();
        let s2 = "c#d#".to_string();
        let res = l844_backspace_string_compare::Solution::backspace_compare(s1, s2);
        assert_eq!(res, true);
    }
    #[test]
    fn large_group_positions() {
        let s1 = "abbxxxxzzy".to_string();
        let res = l830_large_group_positions::Solution::large_group_positions(s1);
        let want = vec![vec![3,6]];
        assert_eq!(res, want);
    }
}
