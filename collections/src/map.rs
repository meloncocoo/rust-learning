use std::collections::{HashMap, HashSet};

pub fn base() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("value of {team_name} is {score}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！

    update();
    insert();
    word_counter();
    employee();
    get_median();
    get_pig_latin();
}

fn update() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("Updated: {scores:?}")
}

fn insert() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("Insert when not exists: {scores:?}");
}

fn word_counter() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

/**
 * 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
 * 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
 * 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
 */
fn add_employee_to_department(company: &mut HashMap<String, HashSet<String>>, command: &str) {
    let mut parts = command.split_whitespace();
    if let (Some(name), Some(department), Some(_to)) = (parts.next(), parts.nth(1), parts.next()) {
        company
            .entry(department.to_owned())
            .or_insert_with(HashSet::new)
            .insert(name.to_owned());
        println!("Employee '{}' added to department '{}'", name, department);
    } else {
        println!("Invalid command format. Usage: 'Add <Name> to <Department>'");
    }
}

fn get_department_employees(company: &HashMap<String, HashSet<String>>, department: &str) {
    if let Some(employees) = company.get(department) {
        println!("Employees in department '{}': {:?}", department, employees);
    } else {
        println!("Department '{}' not found", department);
    }
}

fn get_all_employees_sorted(company: &HashMap<String, HashSet<String>>) {
    let mut all_employees = Vec::new();
    for employees in company.values() {
        all_employees.extend(employees.iter());
    }
    all_employees.sort();

    println!("All employees:");
    for employee in all_employees {
        println!("{}", employee);
    }
}

fn employee() {
    let mut company: HashMap<String, HashSet<String>> = HashMap::new();

    add_employee_to_department(&mut company, "Add Sally to Engineering");
    add_employee_to_department(&mut company, "Add Amir to Sales");

    get_department_employees(&company, "Engineering");

    get_all_employees_sorted(&company);
}

/**
 * 使用 Rust 实现给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
 */
fn median(numbers: &[i32]) -> Option<f64> {
    let length = numbers.len();
    if length == 0 {
        return None;
    }

    let mut sorted_numbers: Vec<i32> = numbers.to_vec();
    sorted_numbers.sort();

    if length % 2 == 0 {
        let mid = length / 2;
        Some((sorted_numbers[mid - 1] + sorted_numbers[mid]) as f64 / 2.0)
    } else {
        Some(sorted_numbers[length / 2] as f64)
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();
    for &number in numbers {
        *counts.entry(number).or_insert(0) += 1;
    }

    let (mode, _) = counts.iter().max_by_key(|&(_, count)| count)?;
    Some(*mode)
}

fn get_median() {
    let numbers = vec![5, 2, 7, 2, 3, 8, 2, 4, 6, 4, 9, 2];
    let median_value = median(&numbers).unwrap_or(0.0);
    let mode_value = mode(&numbers).unwrap_or(0);

    println!("Median: {}", median_value);
    println!("Mode: {}", mode_value);
}

/**
 * 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
 * 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
 */

fn pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    if vowels.contains(&first_char) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", chars.as_str(), first_char)
    }
}

fn get_pig_latin() {
    let input = "first apple";
    let words: Vec<&str> = input.split_whitespace().collect();

    let pig_latin_words: Vec<String> = words.into_iter().map(pig_latin).collect();

    let pig_latin_str = pig_latin_words.join(" ");
    println!("{}", pig_latin_str);
}
