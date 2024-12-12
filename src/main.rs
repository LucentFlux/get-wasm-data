use std::collections::BTreeSet;
use std::fs;
use std::io::Write;

fn main() {
    let matches = parse_args();

    let ids: BTreeSet<usize> = matches
        .values_of("data")
        .into_iter()
        .flat_map(|fs| {
            fs.into_iter().map(|f| {
                str::parse::<usize>(f)
                    .unwrap_or_else(|_| panic!("could not parse section ID `{f}`"))
            })
        })
        .collect();

    if ids.is_empty() {
        panic!("expected at least one data section ID to inspect")
    }

    let config = walrus::ModuleConfig::new();
    let path = matches.value_of("input").unwrap();
    let buf = fs::read(&path).unwrap_or_else(|e| panic!("failed to read file {}: {e:#?}", path));
    let module = config.parse(&buf).unwrap();

    let datas: Vec<_> = module
        .data
        .iter()
        .enumerate()
        .filter(|(id, _)| ids.contains(id))
        .map(|(_, data)| data)
        .collect();

    for (data, id) in datas.iter().zip(&ids) {
        println!("data {} has len {}", id, data.value.len());

        if let Some(output) = matches.value_of("output") {
            let mut output = output.to_owned();
            if ids.len() > 1 {
                output = format!("{output}.{id}")
            }
            std::fs::write(&output, &data.value)
                .unwrap_or_else(|e| panic!("failed to emit data to {}: {e:#?}", output));
        } else {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            stdout
                .write_all(&data.value)
                .expect("failed to write data to stdout");
        }
    }
}

fn parse_args() -> clap::ArgMatches<'static> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("The path to write the output data section to. Defaults to stdout."),
        )
        .arg(
            clap::Arg::with_name("input")
                .required(true)
                .help("The input wasm file containing the data to extract."),
        )
        .arg(clap::Arg::with_name("data").multiple(true).help(
            "The specific data section IDs to output. These must match \
             exactly.",
        ))
        .get_matches()
}
