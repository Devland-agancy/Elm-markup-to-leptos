use super::{counter_instance::CounterInstance, counters::Counters};
use regex::Regex;

pub fn replace_counter_in_line(mut line: &str, counters: &mut Counters) -> String {
    // create regex to match this string ""::::CounterName"
    let insert_regex = Regex::new(r"(::|\+\+|--)::\w+").unwrap();

    let mut new_line = String::from(line);

    let _ = insert_regex.replace_all(line, |caps: &regex::Captures| {
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
    assert_eq!(test4, "inc ⅲ another inc ⅳ fix ⅳ");

    // test double deccrement than normal
    let test4 = replace_counter_in_line(
        "dec --::TestRomanCounter another dec --::TestRomanCounter fix ::::TestRomanCounter dec ar --::TestArabicCounter fix ar ::::TestArabicCounter",
        &mut counters,
    );
    assert_eq!(test4, "dec ⅲ another dec ⅱ fix ⅱ dec ar 0 fix ar 0");
}

#[test]
#[should_panic(expected = "Counter TestArabicCounter is decremented while being 0")]
fn decrement_less_than_0() {
    let mut counters = Counters::new();
    counters.add_counter(CounterInstance::new("TestArabicCounter", "counter"));
    counters.add_counter(CounterInstance::new("TestRomanCounter", "roman_counter"));

    replace_counter_in_line("--::TestArabicCounter", &mut counters);
}
