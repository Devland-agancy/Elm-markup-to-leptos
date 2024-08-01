use crate::counter::counter_types::CounterValueType;

use super::{counter_instance::CounterInstance, counters::Counters};
use regex::{Captures, Regex};

pub fn replace_counter_in_line(line: &str, counters: &mut Counters) -> String {
    // create regex to match this string ""::::CounterName"
    let insert_regex = Regex::new(r"::::\w+").unwrap();
    let insert_inc_reg = Regex::new(r"\+\+::\w+").unwrap();
    let insert_dec_reg = Regex::new(r"--::\w+").unwrap();
    //let regex = Regex::new(r"::::\w+").unwrap();

    let mut new_line = String::from(line);

    let _ = insert_regex.replace_all(line, |caps: &regex::Captures| {
        for counter in counters.counters_list.iter() {
            if caps[0] == format!("::::{}", counter.name) {
                new_line = insert_regex
                    .replace(&new_line, &counter.current_value.to_string())
                    .to_string()
            }
        }
        ""
    });

    let _ = insert_inc_reg.replace_all(line, |caps: &regex::Captures| {
        for counter in counters.counters_list.iter_mut() {
            if caps[0] == format!("++::{}", counter.name) {
                counter.current_value.increment(&counter.counter_type);
                new_line = insert_inc_reg
                    .replace(&new_line, &counter.current_value.to_string())
                    .to_string()
            }
        }
        ""
    });

    new_line
}

//create a test for replacecounter_in_line function
#[test]
fn test_replace_counter_in_line() {
    let mut counters = Counters::new();
    counters.add_counter(CounterInstance::new("TestArabicCounter", "counter"));
    counters.add_counter(CounterInstance::new("TestRomanCounter", "roman_counter"));

    //with text after
    let test1 =
        replace_counter_in_line("askljdklasj ::::TestArabicCounter qweqweqwe", &mut counters);
    assert_eq!(test1, "askljdklasj 0 qweqweqwe");

    //no text after
    let test2 = replace_counter_in_line("askljdklasj ::::TestArabicCounter", &mut counters);
    assert_eq!(test2, "askljdklasj 0");

    // 2 same type counters in 1 line
    let test3 = replace_counter_in_line(
        "askljdklasj ::::TestArabicCounter hi ::::TestArabicCounter",
        &mut counters,
    );
    assert_eq!(test3, "askljdklasj 0 hi 0");

    // 2 different type counters in 1 line
    let test4 = replace_counter_in_line(
        "askljdklasj ::::TestArabicCounter hi ::::TestRomanCounter",
        &mut counters,
    );
    assert_eq!(test4, "askljdklasj 0 hi ⅰ");

    // test increment
    let test4 = replace_counter_in_line(
        "askljdklasj ++::TestArabicCounter hi ++::TestRomanCounter",
        &mut counters,
    );
    assert_eq!(test4, "askljdklasj 1 hi ⅱ");

    // test double increment than normal
    let test4 = replace_counter_in_line(
        "inc ++::TestRomanCounter another inc ++::TestRomanCounter fix ::::TestRomanCounter",
        &mut counters,
    );
    assert_eq!(test4, "inc ⅲ another ⅳ fix ⅳ");
}

// fn test_increment_counters() {
//   let mut counters = Counters::new();
//   counters.add_counter(CounterInstance::new("TestArabicCounter", "counter"));
//   counters.add_counter(CounterInstance::new("TestRomanCounter", "roman_counter"));

//   //with text after
//   let test1 =
//       replace_counter_in_line("askljdklasj ::::TestArabicCounter qweqweqwe", &mut counters);
//   assert_eq!(test1, "askljdklasj 0 qweqweqwe");

// }
