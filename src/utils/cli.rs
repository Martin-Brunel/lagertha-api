use std::io;
use neofiglet::FIGfont;

/// ### Cli utils struct
/// 
/// methods to display informations in the command line interface
/// - title
/// - separator
/// - empty_line
/// - prompt
/// - write
pub struct CLIUtils;


impl CLIUtils {
    /// ### Cli title
    /// 
    /// display a title with neofiglet 
    pub fn title() {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("HB_CYBER_CORE");
        assert!(figure.is_some());
        println!("{}", figure.unwrap());
    }

    /// ### Cli separator
    /// 
    /// a clean cli separator
    pub fn separator() {
        println!("=====================================================================");
    }

    /// ### Cli empty line
    /// 
    /// usefull empty line 
    pub fn empty_line() {
        println!("");
    }

    /// ### Cli prompt
    /// 
    /// use to get input from users
    pub fn prompt(title: &str) -> String {
        let stdin = io::stdin();
        let mut output = String::new();
        println!("{} :", title);
        stdin.read_line(&mut output).unwrap();
        output.pop();
        output
    }

    /// ### Cli write
    /// 
    /// write a simple information on the command line interface
    pub fn write(text: &str) {
        println!("{}", text);
    }
}
