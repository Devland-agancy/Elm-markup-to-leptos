use super::{
    counter_instance::CounterInstance, counter_types::CounterValueType, counters::Counters,
};
use regex::Regex;

pub fn replace_counter_in_line(line: &str, counters: &mut Counters) -> String {
    // create regex to match this string ""::::CounterName"
    let insert_regex = Regex::new(r"(::|\+\+|--)::\w+").unwrap();

    let mut new_line = String::from(line);

    let _ = insert_regex.replace_all(line, |caps: &regex::Captures| {
        let mut counter_names = Vec::new();
        for counter in counters.counters_list.iter() {
            counter_names.push(&counter.name)
        }
        if !counter_names.contains(&&caps[0][4..].to_string()) {
            panic!(
                "Counter {} was used out of scope in line {}",
                caps[0][4..].to_string(),
                line
            );
        }
        for counter in counters.counters_list.iter_mut() {
            if caps[0] == format!("++::{}", counter.name) {
                counter.increment();
            }
            if caps[0] == format!("--::{}", counter.name) {
                counter.decrement();
            }
            if caps[0][2..] == format!("::{}", counter.name) {
                new_line = insert_regex
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
    counters.add_counter(CounterInstance::new("TestArabicCounter", "counter", 0));
    counters.add_counter(CounterInstance::new("TestRomanCounter", "roman_counter", 0));

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
    assert_eq!(test4, "askljdklasj 0 hi 0");

    // test increment
    let test4 = replace_counter_in_line(
        "askljdklasj ++::TestArabicCounter hi ++::TestRomanCounter",
        &mut counters,
    );
    assert_eq!(test4, "askljdklasj 1 hi ⅰ");

    // test double increment than normal
    let test4 = replace_counter_in_line(
        "inc ++::TestRomanCounter another inc ++::TestRomanCounter fix ::::TestRomanCounter",
        &mut counters,
    );
    assert_eq!(test4, "inc ⅱ another inc ⅲ fix ⅲ");

    // test double deccrement than normal
    let test4 = replace_counter_in_line(
        "dec --::TestRomanCounter another dec --::TestRomanCounter fix ::::TestRomanCounter dec ar --::TestArabicCounter fix ar ::::TestArabicCounter",
        &mut counters,
    );
    assert_eq!(test4, "dec ⅱ another dec ⅰ fix ⅰ dec ar 0 fix ar 0");
}

#[test]
fn decrement_less_than_0() {
    let mut counters = Counters::new();
    counters.add_counter(CounterInstance::new("TestArabicCounter", "counter", 0));
    counters.add_counter(CounterInstance::new("TestRomanCounter", "roman_counter", 0));

    let test =
        replace_counter_in_line("--::TestArabicCounter --::TestArabicCounter", &mut counters);
    assert_eq!(test, "- -");

    let test2 =
        replace_counter_in_line("::::TestRomanCounter --::TestArabicCounter", &mut counters);
    assert_eq!(test2, "0 -");
}
