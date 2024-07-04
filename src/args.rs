use std::fmt;
use std::fmt::Display;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the directory
    #[arg(short, long, default_value = "./")]
    pub path: String,

    /// Order by a column
    #[arg(short = 'o', long, default_value_t = OrderBy::Size, value_enum)]
    pub order_by: OrderBy,

    /// Order direction
    #[arg(short = 'd', long, default_value_t = OrderDirection::Descending, value_enum)]
    pub order_direction: OrderDirection,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OrderBy {
    Size,
    Name,
    CreatedDate,
    ModifiedDate,
    Extension,
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderBy::Size => "Size",
            OrderBy::Name => "Name",
            OrderBy::CreatedDate => "CreatedDate",
            OrderBy::ModifiedDate => "ModifiedDate",
            OrderBy::Extension => "Extension",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

impl Display for OrderDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderDirection::Ascending => "Ascending",
            OrderDirection::Descending => "Descending"
        };

        write!(f, "{}", s)
    }
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    args
}