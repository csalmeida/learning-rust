/// This program prints Strings and str types and also includes an example of concatenating a Vec of string literals.
fn main() {
    let title = "Ó Gente Da Minha Terra";
    let composers = Vec::from(["Amália Rodrigues", "Tiago Machado"]);

    println!("Título: {}", title);
    println!("Composição: {}", format_composers(composers));

    let lyrics = format!(
        "
É meu e vosso este fado
Destino que nos amarra
Por mais que seja negado
Às cordas de uma guitarra

Sempre que se ouve um gemido
De uma guitarra a cantar
Fica-se logo perdido
Com vontade de chorar

Ó gente da minha terra
Agora é que eu percebi
Esta tristeza que trago
Foi de vós que recebi

E pareceria ternura
Se me deixasse embalar
Era maior a amargura
Menos triste o meu cantar

Ó gente da minha terra
Agora é que eu percebi
Esta tristeza que trago
Foi de vós que a recebi

Ó gente da minha terra
Agora é que eu percebi
Esta tristeza que trago
Esta tristeza que trago
Foi de vós que a recebi
    "
    );

    println!("{}", lyrics);
}

/// Returns a String with formatted names with a forward slash between them:
/// ```
/// "Amália Rodrigues / Tiago Machado"
/// ```
///
/// It takes the first two elements only, the other ones will be ignored.
///
/// # Arguments
///
/// * `composers` - A Vec of string slices that include the full name of a composer.
///
/// # Examples
///
/// ```rust
/// let composers = Vec::from(["Amália Rodrigues", "Tiago Machado"]);
/// println!("Composed by: {}", format_composers(composers));
/// ```
fn format_composers(composers: Vec<&str>) -> String {
    let concat_composers: String = format!("{} / {}", composers[0], composers[1]);

    return concat_composers;
}
