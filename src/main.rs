mod format;
mod int;
mod parse;

use clap::{AppSettings, ArgGroup, Parser};
use colored::Colorize;
use int::{Integer, IntegerOptions};

const MAX_BITS: usize = u64::BITS as usize;

const HELP_TEMPLATE: &str = r"{before-help}{bin} {version}
{author}

{about}

{usage-heading}
    {usage}

{all-args}{after-help}";

/// The clap app for the application.
#[derive(Clone, Debug, Parser)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("format")
        .required(false)
        .args(&["binary", "octal", "decimal", "hexadecimal", "radix"]),
))]
#[clap(group(
    ArgGroup::new("number-bits")
        .required(false)
        .args(&["bytes", "bits", "byte", "short", "int", "long"]),
))]
#[clap(group(
    ArgGroup::new("extend")
        .required(false)
        .args(&["zero-extend", "sign-extend"]),
))]
#[clap(setting(AppSettings::AllowHyphenValues))]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
#[clap(help_template(HELP_TEMPLATE))]
struct App {
    /// The integer to inspect.
    ///
    /// The input is assumed to be a decimal integer unless it starts
    /// with `0b` (binary), `0[o|O]` (octal), `0x` (hex), or
    /// an appropriate radix flag is set.
    ///
    /// The default bit size is equal to the minimum number of bits required
    /// to represent the input.
    ///
    /// The integer is treated as unsigned by default.
    input: String,
    /// Treat the input as a signed integer.
    ///
    /// The number is treated as *UNsiged* by default.
    #[clap(short, long)]
    signed: bool,
    /// Treat the input as an 8-bit integer.
    #[clap(short = 'B', long, alias = "u8", alias = "char")]
    byte: bool,
    /// Treat the input as a 16-bit integer.
    #[clap(short = 'S', long, alias = "u16")]
    short: bool,
    /// Treat the input as a 32-bit integer.
    #[clap(short, long, alias = "u32")]
    int: bool,
    /// Treat the input as a 64-bit integer.
    #[clap(short, long, alias = "u64")]
    long: bool,
    /// Treat the input as integer of specified bytes.
    #[clap(long)]
    bytes: Option<usize>,
    /// Treat the input as integer of specified bits.
    ///
    /// The default bit size is equal to the minimum number of bits required
    /// to represent the input.
    #[clap(short, long)]
    bits: Option<usize>,
    /// Sign-extend the input integer when converting it to a bigger size.
    ///
    /// This is the default if the number is signed.
    #[clap(short = 'e', long, alias = "sext")]
    sign_extend: bool,
    /// Zero-extend the input integer when converting it to a bigger size.
    ///
    /// This is the default if the number is *not* signed.
    #[clap(long, alias = "zext")]
    zero_extend: bool,
    /// Treat the input as a binary (base-2) integer.
    #[clap(short = 'n', long, alias = "bin")]
    binary: bool,
    /// Treat the input as an octal (base-8) integer.
    #[clap(short, long, alias = "oct")]
    octal: bool,
    /// Treat the input as a decimal (base-10) integer.
    ///
    /// It is the default radix if the input does not start with a recognised prefix.
    #[clap(short, long, alias = "dec")]
    decimal: bool,
    /// Treat the input as a hexadecimal (base-16) integer.
    #[clap(short = 'x', long, alias = "hex")]
    hexadecimal: bool,
    /// Treat the input as a integer of the specified radix.
    ///
    /// The input is assumed to be a decimal integer unless it starts
    /// with `0b` (binary), `0[o|O]` (octal), or `0x` (hex).
    #[clap(short, long)]
    radix: Option<u32>,
    /// Show information about the two's complement of the input as well.
    ///
    /// Disabled by default.
    #[clap(short, long, alias = "two")]
    twos_complement: bool,
}

/// Returns the (optional) number of bits specified by the user.
const fn total_bits(app: &App) -> Option<usize> {
    if let Some(app_bits) = app.bits {
        Some(app_bits)
    } else if let Some(bytes) = app.bytes {
        Some(bytes * 8)
    } else if app.byte {
        Some(8)
    } else if app.short {
        Some(16)
    } else if app.int {
        Some(32)
    } else if app.long {
        Some(64)
    } else {
        None
    }
}

/// Returns the (optional) radix specified by the user.
const fn radix(app: &App) -> Option<u32> {
    if app.radix.is_some() {
        app.radix
    } else if app.binary {
        Some(2)
    } else if app.octal {
        Some(8)
    } else if app.decimal {
        Some(10)
    } else if app.hexadecimal {
        Some(16)
    } else {
        None
    }
}

/// Prints information about the integer to the standard output.
fn print_integer_info(integer: &Integer<u64>) {
    println!("Decimal         =>  {}", integer.to_string().blue());
    println!(
        "Binary          =>  {}{}",
        "0b".yellow(),
        format!("{:b}", integer).blue(),
    );
    println!(
        "Octal           =>  {}{}",
        "0o".green(),
        format!("{:o}", integer).blue(),
    );
    println!(
        "Hexadecimal     =>  {}{}",
        "0x".purple(),
        format!("{:x}", integer).blue(),
    );
}

/// Runs the app.
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    let radix = radix(&app)
        .or_else(|| parse::identify_radix(&app.input))
        .unwrap_or(10);

    if app
        .bits
        .or_else(|| app.bytes.map(|b| b * 8))
        .unwrap_or_default()
        > MAX_BITS
    {
        return Err(format!("number of bits must be less than or equal to {}", MAX_BITS).into());
    }

    let is_negative = app.input.starts_with('-');

    let (int, opt_significant_bits) = if is_negative {
        let (int, _) = parse::parse::<i64>(&app.input, radix)?;

        (
            int as u64,
            Some((i64::BITS - int.leading_ones() + 1) as usize),
        )
    } else {
        parse::parse(&app.input, radix)?
    };

    let significant_bits =
        opt_significant_bits.unwrap_or_else(|| (u64::BITS - int.leading_zeros()) as usize);

    let bit_size = total_bits(&app).unwrap_or(significant_bits);
    let signed = app.signed || is_negative;
    let sign_extend = !app.zero_extend && (app.sign_extend || signed);

    let mut integer = Integer::new(
        int as u64,
        IntegerOptions::new(signed, bit_size, Some(significant_bits), sign_extend),
    );

    print_integer_info(&integer);

    println!("\nBits: {}", integer.bits().to_string().cyan().bold());

    if app.twos_complement {
        integer.make_twos_complement();

        println!("\n{}", "2's Complement \\".bright_cyan().bold());
        print_integer_info(&integer);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}: {}", "error".red().bold(), err);
    }
}
