// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

#[derive(Debug)]
struct Worker<'a> {
    name: &'a str,
    title: &'a str,
}

struct CSVData<'a> {
    inner: Vec<Worker<'a>>,
}

fn main() {
    let mut csv_data = CSVData { inner: vec![] };

    let rows: Vec<&str> = MOCK_DATA
        .split('\n')
        .skip(1)
        .filter(|row| row.len() > 0)
        .collect();

    for row in rows {
        let cells: Vec<&str> = row.split(',').collect();
        let name = cells[1];
        let title = cells[4];
        let worker = Worker { name, title };

        csv_data.inner.push(worker);
    }

    for worker in csv_data.inner {
        println!("Worker Info: {:?}", worker);
    }
}
