use select::document::Document;
use select::predicate::Class;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use threadpool::ThreadPool;

const PROBLEM_DESCRIPTION_WIDTH: usize = 40;
const BASE_URL: &str = "https://projecteuler.net";

// program error type
#[derive(Debug)]
enum Error {
    Io(io::Error),
    FetchProblemFailed(usize),
    ParseHtml(usize),
    SolutionAlreadyExists(usize),
}

type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FetchProblemFailed(id) => write!(f, "failed to fetch problem {}", id),
            Error::Io(err) => err.fmt(f),
            Error::ParseHtml(id) => write!(f, "failed to parse html for problem {}", id),
            Error::SolutionAlreadyExists(id) => write!(f, "solution already exists {}", id),
        }
    }
}

fn get_url(id: usize) -> String {
    format!("{}/problem={}", BASE_URL, id)
}

fn fetch_problem(id: usize, url: &str) -> Result<String> {
    // fetch the problem page html from projecteuler
    let res = reqwest::get(url).map_err(|_| Error::FetchProblemFailed(id))?;

    // extract the description `<div class="problem_content">{description elements}</div>`
    let problem = Document::from_read(res)
        .map_err(|_| Error::ParseHtml(id))?
        .find(Class("problem_content"))
        .map(|node| node.text())
        .collect();

    Ok(problem)
}

// fetch the descrption for a problem and create an empty solution program
// in the `examples` directory
fn create_empty_solution(id: usize) -> Result<()> {
    // store solutions at `examples/p{id:03}.rs`
    let path = Path::new("examples/").join(format!("p{:03}.rs", id));

    // don't overwrite pre-existing solutions
    if path.exists() {
        return Err(Error::SolutionAlreadyExists(id));
    }

    // create the problem url and fetch its description
    let url = get_url(id);
    let problem = fetch_problem(id, &url)?;

    // create the solution program
    let mut file = File::create(path)?;
    writeln!(
        file,
        "/*\n[{}]\n{}\n*/\n\n{}",
        url,
        textwrap::fill(&problem, PROBLEM_DESCRIPTION_WIDTH),
        include_str!("../examples/empty_solution.rs")
    )?;

    Ok(())
}

fn main() {
    let pool = ThreadPool::default();

    let ids = 0..100;

    // concurrently create an empty solution for each program
    for id in ids {
        pool.execute(move || match create_empty_solution(id) {
            Ok(()) => println!("empty solution created {}", id),
            Err(e) => eprintln!("error: {}", e),
        });
    }

    pool.join();
}
