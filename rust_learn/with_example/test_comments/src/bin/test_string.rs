fn main() {
    // 通过转义，可以用十六进制值来表示字节。
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    // 也可以使用 Unicode 码位表示。
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->    \
                        <- can be escaped too!";
    println!("{}", long_string);
}