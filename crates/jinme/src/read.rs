use crate::prelude::*;
use std::sync::Arc;
use tracing::info;

/*
pub type Result<'error, T> = ::core::result::Result<T, self::Error<'error>>;
pub type Error<'error> = Box<dyn ::core::error::Error + 'error>;

pub type ReadInput<'input> = &'input str;
pub type ReadOutput = PtrValue;
pub type ReadResult<'error> = Result<'error, ReadOutput>;

pub fn read_one(input: ReadInput) -> ReadResult {
    let (_input, v) = parse::try_parse_one(input)?;
    Ok(v)
}
*/

/// Represents an incomplete read operation.
///
/// This occurs when the input string is not a complete expression and more input
/// is needed to complete parsing.
#[derive(Clone, Debug)]
pub struct IncompleteRead<'a> {
    /// The remaining input that could not be parsed
    pub input: &'a str,
}

/// Represents a complete read operation.
///
/// This occurs when the input string has been successfully parsed into a value.
#[derive(Clone, Debug)]
pub struct CompleteRead<'a> {
    /// The original input string
    pub input: &'a str,
    /// The remaining input after parsing (everything after the parsed expression)
    pub rest_input: &'a str,
    /// The parsed value
    pub value: PtrValue,
}

/// The result of a read operation.
///
/// This enum represents the possible outcomes of parsing input:
/// - `Incomplete` - More input is needed to complete the expression
/// - `Complete` - The expression was successfully parsed
/// - `EndOfInput` - The input ended at a valid point (e.g., empty string)
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let result = read_one("42");
/// match result {
///     ReadOutput::Complete(CompleteRead { value, .. }) => {
///         assert_eq!(value, Value::integer(42));
///     }
///     _ => panic!("Expected complete read"),
/// }
/// ```
#[derive(Clone, Debug)]
pub enum ReadOutput<'input> {
    Incomplete(IncompleteRead<'input>),
    Complete(CompleteRead<'input>),
    EndOfInput,
}

impl<'input> ReadOutput<'input> {
    /// Returns `true` if this is an `EndOfInput` variant
    pub fn is_end_of_input(&self) -> bool {
        matches!(self, Self::EndOfInput)
    }

    /// Attempts to convert this to `EndOfInput`, returning `None` if not an `EndOfInput`
    pub fn try_into_end_of_input_read(self) -> Option<()> {
        if let Self::EndOfInput = self {
            Some(())
        } else {
            None
        }
    }

    /// Converts to `EndOfInput` or panics if not an `EndOfInput`
    pub fn into_end_of_input_read_or_panic(self) -> () {
        let expect_msg = format!("Expected EndOfInput variant but got: {:?}", self);
        self.try_into_end_of_input_read().expect(&expect_msg)
    }

    /// Returns `true` if this is an `Incomplete` variant
    pub fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete(_))
    }

    /// Attempts to convert this to `IncompleteRead`, returning `None` if not `Incomplete`
    pub fn try_into_incomplete_read(self) -> Option<IncompleteRead<'input>> {
        if let Self::Incomplete(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts to `IncompleteRead` or panics if not `Incomplete`
    pub fn into_incomplete_read_or_panic(self) -> IncompleteRead<'input> {
        let expect_msg = format!("Expected Incomplete variant but got: {:?}", self);
        self.try_into_incomplete_read().expect(&expect_msg)
    }

    /// Returns `true` if this is a `Complete` variant
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete(_))
    }

    /// Attempts to convert this to `CompleteRead`, returning `None` if not `Complete`
    pub fn try_into_complete_read(self) -> Option<CompleteRead<'input>> {
        if let Self::Complete(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts to `CompleteRead` or panics if not `Complete`
    pub fn into_complete_read_or_panic(self) -> CompleteRead<'input> {
        let expect_msg = format!("Expected Complete variant but got: {:?}", self);
        self.try_into_complete_read().expect(&expect_msg)
    }

    /// Creates an `Incomplete` read output
    pub fn incomplete(input: &'input str) -> Self {
        Self::Incomplete(IncompleteRead { input })
    }

    /// Creates a `Complete` read output
    pub fn complete(input: &'input str, rest_input: &'input str, value: PtrValue) -> Self {
        Self::Complete(CompleteRead {
            input,
            rest_input,
            value,
        })
    }
}

#[derive(Debug)]
pub struct UnexpectedEndOfInputError<'input>(&'input str);

impl<'input> UnexpectedEndOfInputError<'input> {
    pub fn input(&self) -> &str {
        self.0
    }
}

#[derive(Debug)]
pub struct TypeErasedError<'input>(&'input str, Box<dyn std::error::Error + 'input>);

impl<'input> TypeErasedError<'input> {
    pub fn input(&self) -> &str {
        self.0
    }
    pub fn error(&self) -> &(dyn std::error::Error + 'input) {
        &*self.1
    }
}

#[derive(Debug)]
pub enum ReadError<'input> {
    UnexpectedEndOfInput(UnexpectedEndOfInputError<'input>),
    TypeErased(TypeErasedError<'input>),
}

impl<'input> ReadError<'input> {
    pub fn is_unexpected_end_of_input(&self) -> bool {
        matches!(self, Self::UnexpectedEndOfInput(_))
    }

    pub fn try_into_unexpected_end_of_input(
        self,
    ) -> Result<UnexpectedEndOfInputError<'input>, Self> {
        if let Self::UnexpectedEndOfInput(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    pub fn into_unexpected_end_of_input_or_panic(self) -> UnexpectedEndOfInputError<'input> {
        let expect_msg = format!("Expected UnexpectedEndOfInput variant but got: {:?}", self);
        self.try_into_unexpected_end_of_input().expect(&expect_msg)
    }

    pub fn is_type_erased(&self) -> bool {
        matches!(self, Self::TypeErased(_))
    }

    pub fn try_into_type_erased(self) -> Result<TypeErasedError<'input>, Self> {
        if let Self::TypeErased(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    pub fn into_type_erased_or_panic(self) -> TypeErasedError<'input> {
        let expect_msg = format!("Expected TypeErased variant but got: {:?}", self);
        self.try_into_type_erased().expect(&expect_msg)
    }

    pub fn unexpected_end_of_input(input: &'input str) -> Self {
        Self::UnexpectedEndOfInput(UnexpectedEndOfInputError(input))
    }

    pub fn type_erased(input: &'input str, error: Box<dyn std::error::Error + 'input>) -> Self {
        Self::TypeErased(TypeErasedError(input, error))
    }
}

/// Reads a single value from the input string, returning the value and the remaining unparsed input.
///
/// Being unable to read a value (e.g., due to end of input) is considered an error.
/// Use-sites can convert that to `Option<Value>` as desired.
///
/// # Arguments
///
/// * `env` - The environment to use for symbol resolution
/// * `input` - The input string to parse
///
/// # Returns
///
/// A `Result` containing either a `ReadOutput` or a `ReadError`
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let env = Environment::builder().build_ptr();
/// let result = read_one(env, "42");
/// match result {
///     Ok(ReadOutput::Complete(CompleteRead { value, .. })) => {
///         assert_eq!(value, Value::integer(42));
///     }
///     _ => panic!("Expected complete read"),
/// }
/// ```
pub fn read_one<'input>(
    env: PtrEnvironment,
    input: &'input str,
) -> Result<ReadOutput<'input>, ReadError<'input>> {
    if input.trim().is_empty() {
        return Ok(ReadOutput::EndOfInput);
    }
    match parse::try_parse_one(env, input) {
        Ok((rest_input, value)) => Ok(ReadOutput::complete(input, rest_input, value)),
        Err(nom::Err::Incomplete(nom::Needed::Unknown)) => Ok(ReadOutput::incomplete(input)),
        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::IsA,
        })) if input.is_empty() => Err(ReadError::unexpected_end_of_input("")),
        Err(nom::Err::Error(err)) => Err(ReadError::type_erased(input, Box::new(err))),
        Err(err) => Err(ReadError::type_erased(input, Box::new(err))),
    }
}

/// Reads a single value from the input string, returning the remaining input and the value.
///
/// This is a convenience wrapper around `read_one_v2_inner` that provides tracing
/// and returns a simpler tuple format.
///
/// # Arguments
///
/// * `env` - The environment to use for symbol resolution
/// * `input` - The input string to parse
///
/// # Returns
///
/// A `Result` containing either `(remaining_input, Some(value))` or `(remaining_input, None)`
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let env = Environment::builder().build_ptr();
/// let (rest, value) = read_one_v2(env, "42").unwrap();
/// assert_eq!(rest, "");
/// assert_eq!(value, Some(Value::integer(42)));
/// ```
#[tracing::instrument(ret, level = "info")]
pub fn read_one_v2<'input>(
    env: PtrEnvironment,
    input: &'input str,
) -> Result<(&'input str, Option<PtrValue>), ReadError<'input>> {
    match read_one_v2_inner(env, input) {
        Ok(ReadOutput::EndOfInput) => {
            info!("end-of-input");
            Ok(("", None))
        }
        Ok(ReadOutput::Complete(CompleteRead {
            input: _,
            rest_input,
            value,
        })) => {
            info!("complete read");
            Ok((rest_input, Some(value)))
        }
        Ok(ReadOutput::Incomplete(IncompleteRead { input })) => {
            info!("incomplete read");
            Ok((input, None))
        }
        Err(err) => {
            info!("read error");
            Err(err)
        }
    }
}

/// Inner implementation of `read_one_v2` with tracing and position tracking.
///
/// This function handles parsing, error reporting, and attaches source position
/// metadata to parsed values.
///
/// # Arguments
///
/// * `env` - The environment to use for symbol resolution
/// * `input` - The input string to parse
///
/// # Returns
///
/// A `Result` containing either a `ReadOutput` or a `ReadError`
#[tracing::instrument(fields(input), ret, level = "info")]
fn read_one_v2_inner<'input>(
    env: PtrEnvironment,
    input: &'input str,
) -> Result<ReadOutput<'input>, ReadError<'input>> {
    use nom::{
        Err as NomErr,
        error::{Error as NomError, ErrorKind as NomErrKind},
    };

    // Handle empty/whitespace-only input early
    if input.trim().is_empty() {
        return Ok(ReadOutput::EndOfInput);
    }

    // Save the original input for position computation
    let original_input = input;
    let buffer = input;

    let result = match parse::try_parse_one(env, input) {
        Ok((rest_input, value)) => {
            info!("complete read");
            // Compute position info based on the input pointers
            let pos = compute_consumed_range(buffer, original_input, rest_input);
            // Attach position to the value
            let value_with_pos = attach_position_to_value(value, pos);
            Ok(ReadOutput::complete(
                original_input,
                rest_input,
                value_with_pos,
            ))
        }
        Err(
            err @ NomErr::Error(NomError {
                input,
                code: NomErrKind::IsA, // | NomErrKind::OneOf
            }),
        ) => {
            let input = input.trim();
            if input.is_empty() {
                info!("end-of-input");
                return Ok(ReadOutput::EndOfInput);
            }
            let first_char = input.chars().next();
            match first_char {
                Some('(' | '[' | '{' | '#') => {
                    info!("unexpected end-of-input");
                    Err(ReadError::unexpected_end_of_input(input))
                }
                _ => {
                    info!("other error");
                    Err(ReadError::type_erased(input, Box::new(err)))
                }
            }
        }
        Err(err) => {
            // todo!("unhandled error: {:?}", err);
            Err(ReadError::type_erased(input, Box::new(err)))
        }
    };

    result
}

#[cfg(test)]
mod v2_tests_inner {
    use super::*;

    fn create_env() -> PtrEnvironment {
        let mut env_builder = Environment::builder();
        env_builder.set_current_namespace_var("clojure.core", "*ns*");
        env_builder.insert_namespace(Namespace::new_empty_ptr("clojure.core"));
        env_builder.build_ptr()
    }

    fn get_source_position_from_value_meta(v: &Value) -> SourcePosition {
        let meta_ref = Option::expect(value::optics::preview_meta_ref(v), "meta should be present");

        let begin_line_key = SourcePosition::begin_line_key();
        let begin_column_key = SourcePosition::begin_column_key();
        let end_line_key = SourcePosition::end_line_key();
        let end_column_key = SourcePosition::end_column_key();

        let begin_line_value = Option::expect(
            meta_ref.get(&begin_line_key),
            &format!("begin-line key should be in meta: {begin_line_key}"),
        );
        let begin_column_value = Option::expect(
            meta_ref.get(&begin_column_key),
            &format!("begin-column key should be in meta: {begin_column_key}"),
        );
        let end_line_value = Option::expect(
            meta_ref.get(&end_line_key),
            &format!("end-line key should be in meta: {end_line_key}"),
        );
        let end_column_value = Option::expect(
            meta_ref.get(&end_column_key),
            &format!("end-column key should be in meta: {end_column_key}"),
        );

        let begin_line_integer = Option::expect(
            value::optics::preview_integer(&begin_line_value),
            &format!("begin_line value should be an integer, but was: {begin_line_value}"),
        );
        let begin_column_integer = Option::expect(
            value::optics::preview_integer(&begin_column_value),
            &format!("begin_column value should be an integer, but was: {begin_column_value}"),
        );
        let end_line_integer = Option::expect(
            value::optics::preview_integer(&end_line_value),
            &format!("end_line value should be an integer, but was: {end_line_value}"),
        );
        let end_column_integer = Option::expect(
            value::optics::preview_integer(&end_column_value),
            &format!("end_column value should be an integer, but was: {end_column_value}"),
        );

        SourcePosition {
            begin_line: begin_line_integer as usize,
            begin_column: begin_column_integer as usize,
            end_line: end_line_integer as usize,
            end_column: end_column_integer as usize,
        }
    }

    #[test]
    fn position_tracking_on_symbol() {
        // - arrange
        let env = create_env();
        let input = "hello";

        // - act
        let read_output = Result::expect(read_one_v2_inner(env, input), "successful read");
        let CompleteRead { value, .. } =
            Option::expect(read_output.try_into_complete_read(), "complete read");

        // - assert
        // Verify position information is in the meta and is correct
        let SourcePosition {
            begin_line,
            begin_column,
            end_line,
            end_column,
        } = get_source_position_from_value_meta(&value);
        assert_eq!(
            begin_line, 1,
            "begin-line should be 1, but was: {begin_line}"
        );
        assert_eq!(
            begin_column, 1,
            "begin-column should be 1, but was: {begin_column}"
        );
        assert_eq!(end_line, 1, "end-line should be 1, but was: {end_line}");
        assert_eq!(
            end_column, 6,
            "end-column should be 6, but was: {end_column}"
        );
    }

    #[test]
    fn position_tracking_on_list() {
        // - arrange
        let env = create_env();
        let input = "(+ 1 2)";
        // - act
        let read_result = read_one_v2_inner(env, input);
        // - assert
        let read_output = read_result.expect("successful read");
        let complete_read = read_output.try_into_complete_read().expect("complete read");
        let value = &complete_read.value;

        // Extract meta from list
        let _ = value::optics::preview_list(value.as_ref()).expect("should be a list");

        // Check position keys exist
        // - assert!(meta.is_some(), "meta should be present on list");
        // let meta_ref = meta.unwrap();
        let meta_ref = value::optics::preview_meta_ref(value.as_ref());
        let meta_ref = Option::unwrap(meta_ref);

        // Verify position information is in the meta
        let begin_line_value = meta_ref.get(&SourcePosition::begin_line_key());
        assert!(begin_line_value.is_some(), "line key should be in meta");
    }

    /*
    #[test]
    fn incomplete_list() {
        // - arrange
        let env = create_env();
        let input = "(prn";
        // - act
        let read_result = read_one_v2_inner(env, input);
        // - assert
        let read_error = read_result.expect_err("expect unsuccessful read");
        let eoi_error = read_error.try_into_unexpected_end_of_input().expect("unexpected end-of-input (reading list)");
        assert_eq!(eoi_error.input(), "(prn");
        //let complete_read = read_output.try_into_incomplete_read().expect("incomplete read");
        //assert!(complete_read.value.is_list());
        //assert!(complete_read.rest_input.is_empty());
    }

    // #[test]
    // fn read_form_split_across_multiple_lines() {
    //     // - arrange
    //     let input = "(prn\n:ok)";
    //     // - act
    //     let read_result = super::read_one_v2_inner(env, input);
    //     // - assert
    //     assert!(read_result.is_ok());
    //     let read_output = read_result.unwrap();
    //     let (rest_input, value) = read_output.unwrap();
    //     assert!(value.is_some());
    //     let value = value.unwrap();
    //     assert!(value.is_list(), "{:?}", value);
    // }
    */
}

pub mod parse {
    use std::sync::Arc;

    use crate::prelude::*;
    use nom::{
        IResult,
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, none_of, one_of},
        combinator::{cut, map, opt, recognize, value},
        multi::{many0, many1, separated_list0},
        sequence::{delimited, preceded, separated_pair, tuple},
    };

    // Type alias for cleaner signatures
    type ParseResult<'a> = IResult<&'a str, PtrValue>;

    // Whitespace parsers don't need environment
    pub fn ws0(input: &str) -> IResult<&str, ()> {
        value((), many0(one_of(", \t\r\n")))(input)
    }

    pub fn ws1(input: &str) -> IResult<&str, ()> {
        value((), many1(one_of(", \t\r\n")))(input)
    }

    // ========== BUILDER FUNCTIONS ==========

    pub fn build_try_parse_one<'o, 'i: 'o>(
        env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_one(env.clone(), input)
    }

    pub fn build_try_parse_nil<'o, 'i: 'o>(
        _env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_nil(input)
    }

    pub fn build_try_parse_boolean<'o, 'i: 'o>(
        _env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_boolean(input)
    }

    pub fn build_try_parse_number<'o, 'i: 'o>(
        _env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_number(input)
    }

    pub fn build_try_parse_string<'o, 'i: 'o>(
        _env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_string(input)
    }

    pub fn build_try_parse_symbol<'o, 'i: 'o>(
        _env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_symbol(input)
    }

    pub fn build_try_parse_keyword<'o, 'i: 'o>(
        _env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_keyword(input)
    }

    pub fn build_try_parse_list<'o, 'i: 'o>(
        env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_list(env.clone(), input)
    }

    pub fn build_try_parse_vector<'o, 'i: 'o>(
        env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_vector(env.clone(), input)
    }

    pub fn build_try_parse_set<'o, 'i: 'o>(
        env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_set(env.clone(), input)
    }

    pub fn build_try_parse_map<'o, 'i: 'o>(
        env: PtrEnvironment,
    ) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_map(env.clone(), input)
    }

    // ========== PARSER FUNCTIONS ==========

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_one(env: PtrEnvironment, input: &'_ str) -> ParseResult<'_> {
        let parser = alt((
            |i| try_parse_nil(i),
            |i| try_parse_boolean(i),
            |i| try_parse_number(i),
            |i| try_parse_string(i),
            |i| try_parse_list(env.clone(), i),
            |i| try_parse_vector(env.clone(), i),
            |i| try_parse_set(env.clone(), i),
            |i| try_parse_map(env.clone(), i),
            |i| try_parse_keyword(i),
            |i| try_parse_symbol(i),
        ));
        preceded(ws0, parser)(input)
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_nil(input: &'_ str) -> ParseResult<'_> {
        let (remaining, _) = tag("nil")(input)?;
        let value = Arc::new(Value::nil());
        Ok((remaining, value))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_boolean(input: &'_ str) -> ParseResult<'_> {
        let mut parser = alt((
            map(tag("true"), |_| Arc::new(Value::boolean(true))),
            map(tag("false"), |_| Arc::new(Value::boolean(false))),
        ));
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_number(input: &'_ str) -> ParseResult<'_> {
        let number_parser = recognize(tuple((
            opt(char('-')),
            many1(one_of("0123456789")),
            opt(tuple((char('.'), many1(one_of("0123456789"))))),
        )));

        let mut parser = map(number_parser, |s: &str| {
            if s.contains('.') {
                let float: f64 = s.parse().unwrap();
                Arc::new(Value::float(float.into()))
            } else {
                Arc::new(Value::integer(s.parse().unwrap()))
            }
        });
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_string(input: &'_ str) -> ParseResult<'_> {
        let backslash_escape = alt((
            map(tag("\\\""), |_| '"'),
            map(tag("\\\\"), |_| '\\'),
            map(tag("\\n"), |_| '\n'),
            map(tag("\\t"), |_| '\t'),
            map(tag("\\r"), |_| '\r'),
        ));

        let string_char = alt((backslash_escape, none_of("\"\\"")));

        let mut parser = delimited(
            char('"'),
            map(many0(string_char), |chars| {
                Arc::new(Value::string(chars.into_iter().collect()))
            }),
            char('"'),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_symbol(input: &'_ str) -> ParseResult<'_> {
        let symbol_charset =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+-=~<>.";
        let build_symbol_chars = || recognize(many1(one_of(symbol_charset)));

        // Try to parse a qualified symbol (namespace/name), falling back to unqualified
        let qualified_parser = map(
            tuple((build_symbol_chars(), char('/'), build_symbol_chars())),
            |(namespace, _, name): (&str, char, &str)| {
                Arc::new(Value::symbol(Symbol::new_qualified(namespace, name)))
            },
        );

        let unqualified_parser = map(build_symbol_chars(), |s: &str| {
            Arc::new(Value::symbol(Symbol::new_unqualified(s)))
        });

        // Also handle "/" as a special unqualified symbol for division
        let slash_parser = map(tag("/"), |_| {
            Arc::new(Value::symbol(Symbol::new_unqualified("/")))
        });

        let mut parser = alt((qualified_parser, unqualified_parser, slash_parser));
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_keyword(input: &'_ str) -> ParseResult<'_> {
        // Charset without : (prefix) and / (namespace separator)
        let keyword_charset =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+-=~<>.";
        let build_keyword_chars = || recognize(many1(one_of(keyword_charset)));

        // Consume the initial `:` or `::`
        let (input, prefix) = alt((tag("::"), tag(":")))(input)?;

        // Check for special case: :/ should be the keyword "/"
        // But ::/ is NOT allowed (:: requires qualified form)
        let (input, special_slash) = opt(tag("/"))(input)?;
        if special_slash.is_some() {
            if prefix == "::" {
                // :: requires qualification, :/ is not qualified, so reject
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Verify,
                )));
            }
            return Ok((
                input,
                Arc::new(Value::keyword(Keyword::new_unqualified("/"))),
            ));
        }

        // Parse the first identifier (either namespace or the whole keyword)
        let (input, first_part) = build_keyword_chars()(input)?;

        // Check for qualified keyword (namespace/name)
        let (input_after_slash, is_qualified) = opt(char('/'))(input)?;
        let (final_input, keyword) = if is_qualified.is_some() {
            let (input_, second_part) = build_keyword_chars()(input_after_slash)?;
            (input_, Keyword::new_qualified(first_part, second_part))
        } else {
            // If prefix is ::, then keyword MUST be qualified
            if prefix == "::" {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Verify,
                )));
            }
            (input, Keyword::new_unqualified(first_part))
        };

        Ok((final_input, Arc::new(Value::keyword(keyword))))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_list(env: PtrEnvironment, input: &'_ str) -> ParseResult<'_> {
        let mut parser = delimited(
            char('('),
            map(
                cut(separated_list0(ws1, build_try_parse_one(env.clone()))),
                List::from,
            ),
            preceded(ws0, char(')')),
        );
        let (remaining, list) = parser(input)?;
        Ok((remaining, Arc::new(Value::list(list))))

        /*
        if list.is_empty() {
            return Ok((remaining, Arc::new(Value::list(list))));
        }
        let head = list.get_first().unwrap();
        let head = match value::optics::preview_symbol(head.as_ref()) {
            None => return Ok((remaining, Arc::new(Value::list(list)))),
            Some(symbol) => symbol,
        };
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let resolve_func = clojure_core.get_function_or_panic("resolve");
        let head = resolve_func.invoke(env.clone(), EvalContext::new_empty(), vec![Arc::new(Value::symbol(head.clone()))]);
        if let Value::Var(var, Some(meta)) = head.as_ref() &&
            meta.get(&Value::keyword_unqualified_ptr("macro")).as_deref()
                .and_then(value::optics::preview_boolean).is_some_and(|boolean| boolean)
        {
            if let Some(macro_impl) = var.deref().as_deref().and_then(value::optics::preview_function_ref) {
                let expanded = macro_impl.invoke(env.clone(), EvalContext::new_empty(), list.iter().skip(1).cloned().collect());
                Ok((remaining, expanded))
            } else {
                Ok((remaining, Arc::new(Value::list(list))))
            }
        } else {
            Ok((remaining, Arc::new(Value::list(list))))
        }
        */
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_vector(env: PtrEnvironment, input: &'_ str) -> ParseResult<'_> {
        let mut parser = delimited(
            char('['),
            map(
                separated_list0(ws1, build_try_parse_one(env.clone())),
                |elements| Arc::new(Value::vector_from(elements)),
            ),
            preceded(ws0, char(']')),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_set(env: PtrEnvironment, input: &'_ str) -> ParseResult<'_> {
        let mut parser = preceded(
            tag("#{"),
            delimited(
                ws0,
                map(
                    separated_list0(ws1, build_try_parse_one(env.clone())),
                    |elements| Arc::new(Value::set_from(elements)),
                ),
                preceded(ws0, char('}')),
            ),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    // #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_map(env: PtrEnvironment, input: &'_ str) -> ParseResult<'_> {
        let mut parser = delimited(
            char('{'),
            map(
                separated_list0(
                    ws1,
                    separated_pair(
                        build_try_parse_one(env.clone()),
                        ws1,
                        build_try_parse_one(env.clone()),
                    ),
                ),
                |pairs| Arc::new(Value::map_from(pairs)),
            ),
            preceded(ws0, char('}')),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_env() -> PtrEnvironment {
        let mut env_builder = Environment::builder();
        env_builder.set_current_namespace_var("clojure.core", "*ns*");
        env_builder.insert_namespace(Namespace::new_empty_ptr("clojure.core"));
        env_builder.build_ptr()
    }

    #[test]
    fn symbol_unqualified() {
        // - arrange
        let env = create_env();
        let input = "hello";
        // - act
        let result = parse::try_parse_symbol(input);
        // - assert
        let (remaining, value) = result.expect("should parse unqualified symbol");
        assert!(remaining.is_empty());
        let symbol = value::optics::preview_symbol(value.as_ref()).expect("should be a symbol");
        assert!(symbol.is_unqualified(), "symbol should be unqualified");
        assert_eq!(symbol.name(), "hello");
        assert_eq!(symbol.namespace(), None);
    }

    #[test]
    fn symbol_qualified() {
        // - arrange
        let env = create_env();
        let input = "foo/bar";
        // - act
        let result = parse::try_parse_symbol(input);
        // - assert
        let (remaining, value) = result.expect("should parse qualified symbol");
        assert!(remaining.is_empty());
        let symbol = value::optics::preview_symbol(value.as_ref()).expect("should be a symbol");
        assert!(symbol.is_qualified(), "symbol should be qualified");
        assert_eq!(symbol.name(), "bar");
        assert_eq!(symbol.namespace(), Some("foo"));
    }

    #[test]
    fn symbol_slash() {
        // - arrange
        let env = create_env();
        let input = "/";
        // - act
        let result = parse::try_parse_symbol(input);
        // - assert
        let (remaining, value) = result.expect("should parse slash as symbol");
        assert!(remaining.is_empty());
        let symbol = value::optics::preview_symbol(value.as_ref()).expect("should be a symbol");
        assert!(symbol.is_unqualified(), "/ should be an unqualified symbol");
        assert_eq!(symbol.name(), "/");
        assert_eq!(symbol.namespace(), None);
    }

    #[test]
    fn keyword_unqualified() {
        // - arrange
        let env = create_env();
        let input = ":hello";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert
        let (remaining, value) = result.expect("should parse unqualified keyword");
        assert!(remaining.is_empty());
        let keyword = value::optics::preview_keyword(value.as_ref()).expect("should be a keyword");
        assert!(keyword.is_unqualified(), "keyword should be unqualified");
        assert_eq!(keyword.name(), "hello");
        assert_eq!(keyword.namespace(), None);
    }

    #[test]
    fn keyword_qualified() {
        // - arrange
        let env = create_env();
        let input = ":foo/bar";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert
        let (remaining, value) = result.expect("should parse qualified keyword");
        assert!(remaining.is_empty());
        let keyword = value::optics::preview_keyword(value.as_ref()).expect("should be a keyword");
        assert!(keyword.is_qualified(), "keyword should be qualified");
        assert_eq!(keyword.name(), "bar");
        assert_eq!(keyword.namespace(), Some("foo"));
    }

    #[test]
    fn keyword_slash() {
        // - arrange
        let env = create_env();
        let input = ":/";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert
        let (remaining, value) = result.expect("should parse slash as keyword");
        assert!(remaining.is_empty());
        let keyword = value::optics::preview_keyword(value.as_ref()).expect("should be a keyword");
        assert!(
            keyword.is_unqualified(),
            ":/ should be an unqualified keyword"
        );
        assert_eq!(keyword.name(), "/");
        assert_eq!(keyword.namespace(), None);
    }

    #[test]
    fn keyword_double_colon_slash_rejected() {
        // - arrange
        let env = create_env();
        let input = "::/";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert
        assert!(
            result.is_err(),
            "::/ should be rejected because :: requires qualified form"
        );
    }

    #[test]
    fn keyword_slash_with_following_identifier_rejected() {
        // - arrange
        let env = create_env();
        let input = ":/foo";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert - either error or has remaining input (unreadable form)
        match result {
            Ok((remaining, _)) => {
                assert!(
                    !remaining.is_empty(),
                    ":/foo is unreadable, should have remaining input or error"
                );
            }
            Err(_) => {
                // Also acceptable - rejection is fine
            }
        }
    }

    #[test]
    fn keyword_double_colon_slash_with_following_identifier_rejected() {
        // - arrange
        let env = create_env();
        let input = "::/foo";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert - should error
        assert!(result.is_err(), "::/foo should be rejected as unreadable");
    }

    #[test]
    fn keyword_qualified_missing_name_rejected() {
        // - arrange
        let env = create_env();
        let input = ":foo/";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert
        assert!(result.is_err(), ":foo/ is unreadable, missing name after /");
    }

    #[test]
    fn keyword_double_colon_qualified_missing_name_rejected() {
        // - arrange
        let env = create_env();
        let input = "::foo/";
        // - act
        let result = parse::try_parse_keyword(input);
        // - assert
        assert!(
            result.is_err(),
            "::foo/ is unreadable, missing name after /"
        );
    }

    // Empty list tests
    #[test]
    fn empty_list() {
        let env = create_env();
        let input = "()";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list");
        assert!(remaining.is_empty());
        let elements = value::optics::preview_list(value.as_ref()).expect("should be a list");
        assert!(elements.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_space() {
        let env = create_env();
        let input = "( )";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with space");
        assert!(remaining.is_empty());
        let elements = value::optics::preview_list(value.as_ref()).expect("should be a list");
        assert!(elements.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_comma() {
        let env = create_env();
        let input = "(,)";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with comma");
        assert!(remaining.is_empty());
        let elements = value::optics::preview_list(value.as_ref()).expect("should be a list");
        assert!(elements.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_comma_space() {
        let env = create_env();
        let input = "(, )";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with comma and space");
        assert!(remaining.is_empty());
        let elements = value::optics::preview_list(value.as_ref()).expect("should be a list");
        assert!(elements.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_space_comma() {
        let env = create_env();
        let input = "( ,)";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with space and comma");
        assert!(remaining.is_empty());
        let list = value::optics::preview_list(value.as_ref()).expect("should be a list");
        assert!(list.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_space_comma_space() {
        let env = create_env();
        let input = "( , )";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) =
            result.expect("should parse empty list with space, comma, and space");
        assert!(remaining.is_empty());
        let list = value::optics::preview_list(value.as_ref()).expect("should be a list");
        assert!(list.is_empty(), "list should be empty");
    }

    // Empty vector tests
    #[test]
    fn empty_vector() {
        let env = create_env();
        let input = "[]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector");
        assert!(remaining.is_empty());
        let vector = value::optics::preview_vector(value.as_ref()).expect("should be a vector");
        assert!(vector.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_space() {
        let env = create_env();
        let input = "[ ]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with space");
        assert!(remaining.is_empty());
        let vector = value::optics::preview_vector(value.as_ref()).expect("should be a vector");
        assert!(vector.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_comma() {
        let env = create_env();
        let input = "[,]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with comma");
        assert!(remaining.is_empty());
        let vector = value::optics::preview_vector(value.as_ref()).expect("should be a vector");
        assert!(vector.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_comma_space() {
        let env = create_env();
        let input = "[, ]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with comma and space");
        assert!(remaining.is_empty());
        let vector = value::optics::preview_vector(value.as_ref()).expect("should be a vector");
        assert!(vector.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_space_comma() {
        let env = create_env();
        let input = "[ ,]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with space and comma");
        assert!(remaining.is_empty());
        let vector = value::optics::preview_vector(value.as_ref()).expect("should be a vector");
        assert!(vector.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_space_comma_space() {
        let env = create_env();
        let input = "[ , ]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) =
            result.expect("should parse empty vector with space, comma, and space");
        assert!(remaining.is_empty());
        let vector = value::optics::preview_vector(value.as_ref()).expect("should be a vector");
        assert!(vector.is_empty(), "vector should be empty");
    }

    // Empty set tests
    #[test]
    fn empty_set() {
        let env = create_env();
        let input = "#{}";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set");
        assert!(remaining.is_empty());
        let set = value::optics::preview_set(value.as_ref()).expect("should be a set");
        assert!(set.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_space() {
        let env = create_env();
        let input = "#{ }";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with space");
        assert!(remaining.is_empty());
        let set = value::optics::preview_set(value.as_ref()).expect("should be a set");
        assert!(set.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_comma() {
        let env = create_env();
        let input = "#{,}";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with comma");
        assert!(remaining.is_empty());
        let set = value::optics::preview_set(value.as_ref()).expect("should be a set");
        assert!(set.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_comma_space() {
        let env = create_env();
        let input = "#{, }";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with comma and space");
        assert!(remaining.is_empty());
        let set = value::optics::preview_set(value.as_ref()).expect("should be a set");
        assert!(set.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_space_comma() {
        let env = create_env();
        let input = "#{ ,}";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with space and comma");
        assert!(remaining.is_empty());
        let set = value::optics::preview_set(value.as_ref()).expect("should be a set");
        assert!(set.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_space_comma_space() {
        let env = create_env();
        let input = "#{ , }";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) =
            result.expect("should parse empty set with space, comma, and space");
        assert!(remaining.is_empty());
        let set = value::optics::preview_set(value.as_ref()).expect("should be a set");
        assert!(set.is_empty(), "set should be empty");
    }

    // Empty map tests
    #[test]
    fn empty_map() {
        let env = create_env();
        let input = "{}";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map");
        assert!(remaining.is_empty());
        let map = value::optics::preview_map(value.as_ref()).expect("should be a map");
        assert!(map.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_space() {
        let env = create_env();
        let input = "{ }";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with space");
        assert!(remaining.is_empty());
        let map = value::optics::preview_map(value.as_ref()).expect("should be a map");
        assert!(map.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_comma() {
        let env = create_env();
        let input = "{,}";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with comma");
        assert!(remaining.is_empty());
        let map = value::optics::preview_map(value.as_ref()).expect("should be a map");
        assert!(map.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_comma_space() {
        let env = create_env();
        let input = "{, }";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with comma and space");
        assert!(remaining.is_empty());
        let map = value::optics::preview_map(value.as_ref()).expect("should be a map");
        assert!(map.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_space_comma() {
        let env = create_env();
        let input = "{ ,}";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with space and comma");
        assert!(remaining.is_empty());
        let map = value::optics::preview_map(value.as_ref()).expect("should be a map");
        assert!(map.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_space_comma_space() {
        let env = create_env();
        let input = "{ , }";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) =
            result.expect("should parse empty map with space, comma, and space");
        assert!(remaining.is_empty());
        let map = value::optics::preview_map(value.as_ref()).expect("should be a map");
        assert!(map.is_empty(), "map should be empty");
    }

    // Tests for parsing keywords followed by collections
    #[test]
    fn keyword_followed_by_list() {
        let env = create_env();
        let input = ":foo(bar)";
        let result = parse::try_parse_keyword(input);
        let (remaining, value) = result.expect("should parse keyword");
        assert_eq!(remaining, "(bar)");
        let keyword = value::optics::preview_keyword(value.as_ref()).expect("should be a keyword");
        assert!(keyword.is_unqualified(), "keyword should be unqualified");
        assert_eq!(keyword.name(), "foo");

        // Parse the remaining list
        let list_result = parse::try_parse_list(env, remaining);
        let (list_remaining, list_value) = list_result.expect("should parse list");
        assert!(list_remaining.is_empty());
        let elements = value::optics::preview_list(list_value.as_ref()).expect("should be a list");
        assert_eq!(elements.len(), 1, "list should contain one element");
        let element = elements.get_first().unwrap();
        let symbol =
            value::optics::preview_symbol(element.as_ref()).expect("element should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }

    #[test]
    fn keyword_followed_by_vector() {
        let env = create_env();
        let input = ":foo[bar]";
        let result = parse::try_parse_keyword(input);
        let (remaining, value) = result.expect("should parse keyword");
        assert_eq!(remaining, "[bar]");
        let keyword = value::optics::preview_keyword(value.as_ref()).expect("should be a keyword");
        assert!(keyword.is_unqualified(), "keyword should be unqualified");
        assert_eq!(keyword.name(), "foo");

        // Parse the remaining vector
        let vector_result = parse::try_parse_vector(env, remaining);
        let (vector_remaining, vector_value) = vector_result.expect("should parse vector");
        assert!(vector_remaining.is_empty());
        let elements =
            value::optics::preview_vector(vector_value.as_ref()).expect("should be a vector");
        assert_eq!(elements.len(), 1, "vector should contain one element");
        let element = elements.get_first().unwrap();
        let symbol =
            value::optics::preview_symbol(element.as_ref()).expect("element should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }

    #[test]
    fn symbol_followed_by_vector() {
        let env = create_env();
        let input = "foo[bar]";
        let result = parse::try_parse_symbol(input);
        let (remaining, value) = result.expect("should parse symbol");
        assert_eq!(remaining, "[bar]");
        let symbol = value::optics::preview_symbol(value.as_ref()).expect("should be a symbol");
        assert!(symbol.is_unqualified(), "symbol should be unqualified");
        assert_eq!(symbol.name(), "foo");

        // Parse the remaining vector
        let vector_result = parse::try_parse_vector(env, remaining);
        let (vector_remaining, vector_value) = vector_result.expect("should parse vector");
        assert!(vector_remaining.is_empty());
        let elements =
            value::optics::preview_vector(vector_value.as_ref()).expect("should be a vector");
        assert_eq!(elements.len(), 1, "vector should contain one element");
        let element = elements.get_first().unwrap();
        let symbol =
            value::optics::preview_symbol(element.as_ref()).expect("element should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }

    #[test]
    fn symbol_followed_by_list() {
        let env = create_env();
        let input = "foo(bar)";
        let symbol_result = parse::try_parse_symbol(input);
        let (remaining, value) = symbol_result.expect("should parse symbol");
        assert_eq!(remaining, "(bar)");
        let symbol = value::optics::preview_symbol(value.as_ref()).expect("should be a symbol");
        assert!(symbol.is_unqualified(), "symbol should be unqualified");
        assert_eq!(symbol.name(), "foo");

        // Parse the remaining list
        let list_result = parse::try_parse_list(env, remaining);
        let (list_remaining, list_value) = list_result.expect("should parse list");
        assert!(list_remaining.is_empty());
        let list = value::optics::preview_list(list_value.as_ref()).expect("should be a list");
        assert_eq!(list.len(), 1, "list should contain one element");
        let element = list.get_first().unwrap();
        let symbol =
            value::optics::preview_symbol(element.as_ref()).expect("element should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }
}
