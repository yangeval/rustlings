fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    // 빌려온 것도 trim할 수 있나?
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    // &str 을 String로 어떻게?
    format!("{} world!", input)
    //input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    // cars -> balloons
    // cars 문자열을 찾고, replace해야 한다. 
    // cars가 있는지도 검토해야 한다.
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
