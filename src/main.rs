use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    //读文件
    let content = fs::read_to_string("code.txt")?;

    //预处理
    let content = trim_white_space(content);

    //关键字集合
    let keywords = get_keywords();

    //变量表
    let mut var_id: Vec<String> = Vec::new();
    //数字表
    let mut num: Vec<String> = Vec::new();

    let res = analysis(&content, &mut var_id, &mut num, keywords);

    println!("++++++Token++++++");
    for token in res {
        println!("{}", token);
    }

    println!("++++++数字表++++++");
    for token in num{
        println!("{}", token);
    }

    println!("++++++变量表++++++");
    for token in var_id{
        println!("{}", token);
    }

    Ok(())
}

//预处理，去掉空格换行
fn trim_white_space(target: String) -> String {
    let mut res = String::new();
    for v in target.split_whitespace() {
        res.push_str(v);
    }
    res
}

fn get_keywords() -> Vec<&'static str> {
    vec![
        "const",
        "var",
        "procedure",
        "begin",
        "end",
        "odd",
        "if",
        "then",
        "call",
        "while",
        "do",
        "read",
        "write",
    ]
}

fn analysis(
    content: &String,
    var_id: &mut Vec<String>,
    num: &mut Vec<String>,
    keywords: Vec<&str>,
) -> Vec<String> {
    let mut buf = String::new();
    let mut res: Vec<String> = Vec::new();
    for ch in content.chars() {
        //先判断是否为关键字
        match ch {
            'a'..='z' => {
                //若以数字开头，遇到了非数字，则是数字类型
                if buf.starts_with(char::is_numeric) {
                    let index = find(num, buf.clone());
                    if index == 10000 {
                        num.push(buf.clone());
                        res.push(format!("<{}, 数字, {}>", buf, num.len()));
                    } else {
                        res.push(format!("<{}, 数字, {}>", buf, index + 1));
                    }
                    buf.clear();
                }

                //若过开头是 + - * / 则表示是没有用+= -= ..这一类的运算符号
                if buf.starts_with(|x| {
                    x == '+' || x == '-' || x == '*' || x == '/' || x == '<' || x == '>'
                }) {
                    res.push(format!("<{}, 运算符>", buf));
                    buf.clear();
                }

                //不是数字类型的话就判断是否为关键字
                buf.push(ch);
                for keyword in &keywords {
                    //若存在关键字表则关键字之前的是变量名,否则是变量
                    if buf.contains(keyword) {
                        let tmp = buf.replace(keyword, "");
                        if tmp.len() > 0 {
                            let index = find(var_id, tmp.clone());
                            if index == 10000 {
                                var_id.push(buf.clone());
                                res.push(format!("<{}, 变量, {}>", tmp, var_id.len()));
                            } else {
                                res.push(format!("<{}, 变量, {}>", tmp, index + 1));
                            }
                        }
                        buf.clear();
                        res.push(format!("<{}, 关键字>", keyword));
                    }
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if buf.starts_with(|x| {
                    x == '+' || x == '-' || x == '*' || x == '/' || x == '<' || x == '>'
                }) {
                    res.push(format!("<{}, 运算符>", buf));
                    buf.clear();
                }
                buf.push(ch);
            }
            //若遇到操作符号
            '+' | '-' | '*' | '/' => {
                //判断buf的内容
                //如果是字母和数字都有，则是变量名，若只有数字，则是数字
                is_var_or_num(&mut buf, &mut res, num, var_id);
                buf.clear();
                buf.push(ch);
            }

            '>' | '<' | ':' => {
                is_var_or_num(&mut buf, &mut res, num, var_id);
                buf.clear();
                buf.push(ch);
            }

            '=' => {
                //若buf是以 + - * / < > : 开头，则是运算符号否则就是赋值的意思
                //兼容<= >= += -= *= /= 这些组合的符号
                is_var_or_num(&mut buf, &mut res, num, var_id);
                buf.push(ch);
                res.push(format!("<{}, 运算符号>", buf));
                buf.clear();
            }

            //不等于
            '#' => {
                is_var_or_num(&mut buf, &mut res, num, var_id);
                res.push(format!("<{}, 运算符>", ch));
            }

            //分割符号,结束符
            ';' | ',' => {
                is_var_or_num(&mut buf, &mut res, num, var_id);
                res.push(format!("<{}, 分割符号>", ch));
                buf.clear();
            }
            '(' | ')' => {
                is_var_or_num(&mut buf, &mut res, num, var_id);
                res.push(format!("<{}, 分割符号>", ch));
                buf.clear();
            }
            '.' => {
                res.push(format!("<{}, 结束符>", ch));
            }
            _ => continue,
        }
    }

    res
}

fn is_var_or_num(buf: &mut String, res: &mut Vec<String>, num: &mut Vec<String>, var_id: &mut Vec<String>) {
    if buf.len() == 0 {
        return;
    }

    let x: &[_] = &[':', '<', '>', '+', '-', '*', '/'];
    if buf.starts_with(x) {
        return;
    }
    //判断buf的内容
    //如果是字母和数字都有，则是变量名，若只有数字，则是数字
    if buf.contains(char::is_numeric) {
        if buf.contains(char::is_alphabetic) {
            let index = find(var_id, buf.clone());
            if index == 10000 {
                var_id.push(buf.clone());
                res.push(format!("<{}, 变量, {}>", buf, var_id.len()));
            } else {
                res.push(format!("<{}, 变量, {}>", buf, index + 1));
            }
            buf.clear();
        } else {
            let index = find(num, buf.clone());
            if index == 10000 {
                num.push(buf.clone());
                res.push(format!("<{}, 数字, {}>", buf, num.len()));
            } else {
                res.push(format!("<{}, 数字, {}>", buf, index + 1));
            }
            buf.clear();
        }
    }
}

fn find<T>(vec: &mut Vec<T>, item: T) -> usize
where
    T: PartialEq,
{
    for (index, it) in vec.iter().enumerate() {
        if *it == item {
            return index;
        }
    }
    10000
}
