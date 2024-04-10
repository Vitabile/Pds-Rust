
use clap::Parser;

/// Simple program to slug a String!
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// String to be slugged
    slug_in: Vec<String>,
    /// Number of iteration
    #[arg(short, long, default_value_t = 1)]
    repeat: u32,
    /// Flag for a verbose output
    #[arg(short, long, default_value_t = false)] 
    verbose: bool,
}

const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

fn main() {
    let args = Args::parse();
    let string = args.slug_in.join(" ");
    if args.verbose{
        println!("Stringa su cui calcolare {} volte lo slug ... {string}",args.repeat);
    }
    for i in 0..args.repeat{
        println!("#{i} slug: {}",slugify(string.as_str()))
    }
    
}


fn slugify(s: &str) -> String {
    let mut converted_string = String::new();

    for character in s.to_lowercase().chars(){
        let converted_char = conv(character);
        if (converted_char == '-' && (converted_string.len() == 0 || converted_string.chars().last().unwrap() != '-'))
            || converted_char != '-'
        {
            converted_string.push(converted_char);
        }
    }
    if converted_string.len() > 1 && converted_string.chars().last().unwrap() == '-'{
        converted_string.pop();
    }
    converted_string
}



fn conv(c: char) -> char {
    if c.is_ascii_alphanumeric() {c} else {
        let iterator = SUBS_I.chars();
        for (index,character) in iterator.enumerate(){
            if character == c {
                return SUBS_O.chars().nth(index).unwrap();
            }
        }
        '-'
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv_lettera_non_accentata() {
        assert_eq!(slugify("a"), "a");
    }
    #[test]
    fn conv_lettera_accentata() {
        assert_eq!(slugify("è"), "e");
    }
    #[test]
    fn conv_lettera_non_ammessa_sconosciuta() {
        assert_eq!(slugify("👌"), "-");
    }

    #[test]
    fn conv_lettera_accentata_non_in_lista() {
        assert_eq!(slugify("ἀ"), "-");
    }

    #[test]
    fn stringa_con_spazi() {
        assert_eq!(slugify("Hello World!"), "hello-world");
    }
    #[test]
    fn stringa_con_caratteri_accentati() {
        assert_eq!(slugify("perchè"), "perche");
    }

    #[test]
    fn stringa_vuota() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn stringa_con_piu_spazi_consecutivi() {
        assert_eq!(slugify("ciao   ciao"), "ciao-ciao");
    }

    #[test]
    fn stringa_con_piu_spazi_non_validi_consecutivi() {
        assert_eq!(slugify("ciao???ciao"), "ciao-ciao");
    }

    #[test]
    fn stringa_con_solo_caratteri_non_validi() {
        assert_eq!(slugify("???"), "-");
    }

    #[test]
    fn stringa_con_spazio_alla_fine() {
        assert_eq!(slugify("ciao "), "ciao");
    }
    

}
