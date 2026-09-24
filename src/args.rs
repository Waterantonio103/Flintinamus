use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    // Target and Directory, REQUIRED
    ///Pattern to search for
    pub target: String,
    #[arg(required = true)]
    ///Directory(s) to search in
    pub directory: Vec<String>,
    //

    //Search conditions, OPTIONAL
    #[arg(long)]
    ///Stop searching for patterns after a certain count
    pub max_count: Option<usize>,
    #[arg(short, long, default_value_t = false)]
    ///Check for whole pattern instances only
    pub whole: bool,
    #[arg(short, long, default_value_t = false)]
    ///Check for insensitive cases
    pub insensitive: bool,
    #[arg(short, long, default_value_t = false)]
    ///Perform a recursive search on the chosen directory
    pub recursive: bool,
    //

    //Display options, OPTIONAL 
    #[arg(short, long, default_value_t = false)]
    ///Display line numbers
    pub line_numbers: bool,
    #[arg(short, long, default_value_t = false)]
    ///Display count
    pub count: bool,
    #[arg(short, long, default_value_t = false)]
    ///Return only matched pattern at line
    pub only: bool,
    #[arg(short = 'v', long, default_value_t = false, conflicts_with = "only")]
    ///Return lines not matched
    pub invert: bool,
    #[arg(long, default_value_t = false, conflicts_with_all = ["invert", "only", "line_numbers", "files_without_matches"])]
    ///Return only files containing one or more matches
    pub files_with_matches: bool,
    #[arg(long, default_value_t = false, conflicts_with_all = ["invert", "only", "line_numbers", "count", "files_with_matches"])]
    ///Return only files no matches
    pub files_without_matches: bool,
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["invert", "only", "line_numbers", "count", "files_with_matches", "files_without_matches"])]
    ///Print nothing if matched at least once
    pub quiet: bool,
    #[arg(long)]
    ///Lines to display before and after matched line
    pub context: Option<usize>,
    #[arg(long)]
    ///Lines to display before matched line
    pub before_context: Option<usize>,
    #[arg(long)]
    ///Lines to display after matched line
    pub after_context: Option<usize>,
    //
    
}

#[derive(Debug, Clone, Copy)]
pub enum Context {
    Full(Option<usize>),
    Right(Option<usize>),
    Left(Option<usize>),
}

// can use conflicts_with = "other_tag_name" for tags that cant work together
