use teloxide::utils::markdown;

pub fn bold(text: &str) -> String {
    markdown::bold(markdown::escape(text).as_str())
}

pub fn italic(text: &str) -> String {
    markdown::italic(markdown::escape(text).as_str())
}

pub fn underline(text: &str) -> String {
    markdown::underline(markdown::escape(text).as_str())
}
