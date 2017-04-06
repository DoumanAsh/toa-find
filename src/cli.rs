use ::regex;
use regex::Regex;

use ::str::FromStr;
use ::env;
use ::error;
use ::fmt;

const USAGE: &'static str = "USAGE: toa-find [options] <pattern> -- [path]..

Kawaii Toa shall find all your files recursively.

Arguments:
  <pattern> - Regular expression to filter entries by.
  [path]..  - Directory to search. By default current directory is searched.

Options:
  -h, --help         - Prints this message.
  -s, --sym          - Follow symbolic links. By default they are not followed.
      --minhop <num> - Minimum number of hops before starting to look.
      --hop <num>    - Specifies depth of recursion.
  -q, --quiet        - Print only results. Errors are ignored.

By default every type of file system entry is printed.
Below flags can be used to disable defaults and print only particular types of entries.

Entries filters:
  -d, --dir          - Prints directories.
  -f, --file         - Prints files.
";

fn parse_next_int<T: FromStr>(arg: Option<&str>, opt_name: &str) -> Result<T, ParseError> {
    if let Some(num) = arg {
        if let Ok(num) = num.parse::<T>() {
            Ok(num)
        }
        else {
            Err(ParseError(format!("Invalid number {} is supplied for option {}", num, opt_name)))
        }
    }
    else {
        Err(ParseError(format!("Missing value for option {}", opt_name)))
    }
}

#[derive(Debug)]
pub struct ParseError(String);

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Wrong arguments"
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == "" {
            write!(f, "{}", USAGE)
        }
        else {
            write!(f, "ERROR: {}\n\n{}", self.0, USAGE)
        }
    }
}

#[derive(Default)]
pub struct Flags {
    ///Flag whether to print directories or not.
    pub dir: bool,
    ///Flag whether to print executables or not.
    pub file: bool,
    ///Flag whether to follow symbolic links.
    pub sym: bool,
    ///Flag whether to ignore errrors or not.
    pub quiet: bool
}

pub struct Options {
    ///Hop range (min, max)
    pub hop: (usize, usize)
}

impl Default for Options {
    fn default() -> Self {
        Options {
            hop: (0, ::std::usize::MAX)
        }
    }
}

pub struct Parser {
    pub flags: Flags,
    pub opts: Options,
    pub pattern: Regex,
    pub paths: Vec<String>
}

impl Parser {
    pub fn new() -> Result<Option<Parser>, ParseError> {
        Parser::from_args(env::args().skip(1))
    }

    pub fn from_args<S: AsRef<str>, I: IntoIterator<Item=S>>(args: I) -> Result<Option<Parser>, ParseError> {
        let mut flags = Flags::default();
        let mut options = Options::default();
        let mut pattern: Option<Regex> = None;
        let mut paths = Vec::with_capacity(1);

        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            let arg = arg.as_ref();
            if arg.starts_with('-') {
                let opt = &arg[1..];
                match opt {
                    "h" | "-help" => return Ok(None),
                    "d" | "-dir" => flags.dir = true,
                    "f" | "-file" => flags.file = true,
                    "s" | "-sym" => flags.sym = true,
                    "q" | "-quiet" => flags.quiet = true,
                    "-minhop" => {
                        match parse_next_int(args.next().as_ref().map(|val| val.as_ref()), arg) {
                            Ok(num) => options.hop.0 = num,
                            Err(error) => return Err(error)
                        }
                    }
                    "-hop" => {
                        match parse_next_int(args.next().as_ref().map(|val| val.as_ref()), arg) {
                            Ok(num) => options.hop.1 = num,
                            Err(error) => return Err(error)
                        }
                    }
                    "-" => {
                        while let Some(arg) = args.next() {
                            paths.push(arg.as_ref().to_string())
                        }
                    },
                    arg @ _ => return Err(ParseError(format!("Invalid option '{}'", arg)))
                }
            }
            else {
                if pattern.is_some() {
                    return Err(ParseError("Cannot use more than one pattern for now. Gomen, onii-chan :(".to_string()));
                }

                pattern = match regex::Regex::new(arg) {
                    Ok(regex) => Some(regex),
                    Err(error) => return Err(ParseError(format!("Couldn't compile pattern. {}", error)))
                }
            }
        }

        let pattern = match pattern {
            Some(regex) => regex,
            None => return Err(ParseError("Search pattern is missing".to_string()))
        };

        if !flags.dir && !flags.file {
            flags.dir = true;
            flags.file = true;
        }

        if paths.len() == 0 {
            paths.push(".".to_string());
        }

        Ok(Some(Parser {
            flags: flags,
            opts: options,
            pattern: pattern,
            paths: paths
        }))
    }

    pub fn usage() -> &'static str {
        return USAGE;
    }
}


#[cfg(test)]
mod tests {
    use super::{
        parse_next_int,
        ParseError,
        Parser
    };

    #[test]
    fn args_w_pattern_and_paths() {
        let args = [".exe", "--", "path1", "path2"];
        let result = Parser::from_args(&args);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());
        let result = result.unwrap();

        assert!(result.flags.file);
        assert!(result.flags.dir);
        assert_eq!(result.pattern.as_str(), ".exe");
        assert_eq!(result.paths, &args[2..]);
    }

    #[test]
    fn args_w_no_pattern() {
        let args = ["--", "path1", "path2"];
        let result = Parser::from_args(&args);

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(format!("{}", error), format!("{}", ParseError("Search pattern is missing".to_string())));
    }

    #[test]
    fn args_w_pattern() {
        let args = [".*"];
        let result = Parser::from_args(&args);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());
        let result = result.unwrap();

        assert!(result.flags.file);
        assert!(result.flags.dir);
        assert_eq!(result.pattern.as_str(), ".*");
    }

    #[test]
    fn args_w_pattern_only_exe() {
        let args = ["-f", ".*"];
        let result = Parser::from_args(&args);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());
        let result = result.unwrap();

        assert!(result.flags.file);
        assert!(!result.flags.dir);
        assert_eq!(result.pattern.as_str(), ".*");
    }

    #[test]
    fn args_w_few_patterns() {
        let args = ["-f", ".*", "test.*"];
        let result = Parser::from_args(&args);

        assert!(result.is_err());
    }

    #[test]
    fn no_args() {
        let args: [&str; 0] = [];
        let result = Parser::from_args(&args);

        assert!(result.is_err());
    }

    #[test]
    fn bad_arg() {
        let args = ["-bad"];
        let result = Parser::from_args(&args);

        assert!(result.is_err());
    }

    #[test]
    fn parse_num_fail() {
        let opt_name = "my_opt";
        let num_str = "l55";
        let result = parse_next_int::<usize>(Some(num_str), opt_name);

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(format!("{}", error), format!("{}", ParseError(format!("Invalid number {} is supplied for option {}", num_str, opt_name))));
    }

    #[test]
    fn parse_num_fail_empty() {
        let opt_name = "my_opt";
        let result = parse_next_int::<usize>(None, "my_opt");

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(format!("{}", error), format!("{}", ParseError(format!("Missing value for option {}", opt_name))));
    }

    #[test]
    fn parse_num_ok_unsigned() {
        let opt_name = "my_opt";
        let num = 55;
        let num_str = format!("{}", num);
        let result = parse_next_int::<usize>(Some(&num_str), opt_name);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), num);
    }

    #[test]
    fn parse_num_ok_unsigned_negative() {
        let opt_name = "my_opt";
        let num_str = "-55";
        let result = parse_next_int::<usize>(Some(&num_str), opt_name);

        assert!(result.is_err());
    }

    #[test]
    fn parse_num_ok_signed() {
        let opt_name = "my_opt";
        let num = -55;
        let num_str = format!("{}", num);
        let result = parse_next_int::<isize>(Some(&num_str), opt_name);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), num);
    }

}
