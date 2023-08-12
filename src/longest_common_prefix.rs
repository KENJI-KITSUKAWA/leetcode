pub fn longets_common_prefix(strs: Vec<String>) -> String {
    let len = strs.iter().map(|s| s.len()).into_iter().min().unwrap();

    let mut i = 0;
    loop {
        if i == len {
            break;
        }

        if strs
            .iter()
            .map(|s| &s[i..i + 1])
            .all(|c| c == &strs[0][i..i + 1])
        {
            i += 1;
        } else {
            break;
        }
    }
    return strs[0][0..i].to_string();
}

#[test]
pub fn test_it() {
    let strs: Vec<String> = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let result = longets_common_prefix(strs);
    assert_eq!(result, "fl");
}
