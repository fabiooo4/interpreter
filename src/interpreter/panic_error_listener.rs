use antlr_rust::{error_listener::ErrorListener, recognizer::Recognizer};

pub struct PanicErrorListener;

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for PanicErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as antlr_rust::token_factory::TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&antlr_rust::errors::ANTLRError>,
    ) {
        panic!("{}:{} {}", line, column, msg)
    }
}
