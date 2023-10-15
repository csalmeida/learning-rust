/// # String Examples
/// A number of snippets that cover the basic uses of strings with String and str types.
///
/// Strings can be dealt with slightly different in Rust when compared with other languages. Here's what to remember for now:
/// 1. `String` types are used for strings that need to be modified during runtime. e.g User input.
/// 2. `str` is typically a slice of a string of known size, or a literal that is stored in the binary.
///
/// ### Cases this program aims to cover
///
/// 1. Define a String and an str type.
/// 2. Concatenate a String with a String.
/// 3. Concatenate an str with an str.
/// 4. Concatenate a String with an str.
/// 5. Interporalete values into a new string, for example we might want to separate values with a symbol character, for instance, e.g "first_name / last_name".
/// 6. Use double quotes in string text.
/// 7. Define a character and attach it to a string.
/// 8. Explain what happens in memory when we use the two types.
fn main() {
    let string: String = String::from("Hello");
    let string_literal: &str = "World";

    println!("get_string: {}", get_string(string.clone()));
    println!("get_string_slice: {}", get_string_slice(string_literal));

    // 4. Concatenate a String with an str.
    // Strings can be contactenated with a string slice.
    let concat_string_with_str = string.clone() + string_literal;
    println!("let concat_string_with_str: {}", concat_string_with_str);

    // The plus sign cannot be used to contact an &str to a String.
    // The limitation is present for efficiency reasons. It takes the String on the right and grows it if necessary to add the slice on the right.
    // This takes ownership of the String and borrows the &str, avoiding cloning both values.
    let concat_str_with_string = String::from(string_literal) + &string;
    println!("let concat_str_with_string: {}", concat_str_with_string);

    // 3. Concatenate an str with an str. https://doc.rust-lang.org/stable/std/borrow/trait.ToOwned.html#tymethod.to_owned
    // Creates owned data from borrowed data, usually by cloning.
    let concat_str_with_str = "Olá, ".to_owned() + "Mundo!";
    println!("let concat_str_with_str: {}", concat_str_with_str);

    // 5.Interporalete values into a new string, for example we might want to separate values with a symbol character, for instance, e.g "first_name / last_name"
    let string_interpolation = format!("{} / {}", String::from("Bonjour"), "Monde");
    println!("let string_interpolation: {}", string_interpolation);

    // 6. Use double quotes in string text.
    let double_quotes =
        r#"Then they said, "what is the most important one to you", and I didn't have an answer."#;
    println!("let double_quotes: {}", double_quotes);

    // 7. Define a character and attach it to a string.
    let mut character_concat: String = String::from("The character picked for this string is: ");
    character_concat.push('X'); // Pushes a single character, requires single quotes.
    println!("let character_concat: {}", character_concat);
}

/// The `&str` type is a string slice.
/// A string slice cannot shrink or grow, and it's typically a view into a larger string.
/// For example, a string slice can be "Hello" while the characters it using are taken from the string "Hello, World".
/// Additionally, ownership is not lost in the scope where it's defined, the following code would compile and print "World" twice:
///
/// ```rust
/// fn main() {
///     let value = "World";
///     println!("get_string_slice: {}", get_string_slice(value));
///     println!("let value: {}", value);
/// }
/// ```
///
fn get_string_slice(value: &str) -> &str {
    return value;
}

/// The String type is typically used when a string needs to be modified or the length is not known at compile time.
/// Slices can be created from a String.
fn get_string(value: String) -> String {
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Retrieves the type of the function as an `&str`.
    /// The output is currently used to run a few tests.
    fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test]
    fn is_string_slice_defined() {
        let string_literal = get_string_slice("World");
        assert_eq!(type_of(string_literal), "&str");
    }

    #[test]
    fn is_string_defined() {
        let string = String::from("World");
        assert_eq!(type_of(get_string(string)), "alloc::string::String");
    }
}
