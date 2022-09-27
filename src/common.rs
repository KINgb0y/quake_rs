use crate::api::ApiKey;
use crate::quake::quake::Quake;
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use clap::{App, AppSettings, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/*
  TODO: Comment
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub query: String,
    pub start: i32,
    pub size: i32,
    pub ignore_cache: bool,
    pub start_time: String,
    pub end_time: String,
    pub ip_list: Vec<Value>,
    pub shortcuts: Vec<Value>,
}

/*
  TODO: Comment
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Scroll {
    pub query: String,
    pub size: i32,
    pub ignore_cache: bool,
    pub pagination_id: String,
    pub start_time: String,
    pub end_time: String,
    pub ip_list: Vec<Value>,
    pub shortcuts: Vec<Value>,
}

/*
  TODO: Comment
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub query: String,
    pub start: i32,
    pub size: i32,
    pub ignore_cache: bool,
}

/*
  TODO: Comment
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct AggService {
    pub query: String,
    pub start: i32,
    pub size: i32,
    pub ignore_cache: bool,
    pub aggregation_list: Vec<String>,
}

pub struct ArgParse;

impl ArgParse {
    pub fn parse() {
        let matches = App::new("Quake Command-Line Application")
            .version("2.2.2")
            .author("Author: 360 Quake Team  <quake@360.cn>")
            .about("Dose awesome things.")
            .subcommand(
                SubCommand::with_name("init")
                    .about("Initialize the Quake command-line")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .arg(
                        Arg::with_name("Api_Key")
                            .index(1)
                            .help("Initialize the Quake command-line")
                    )
            )
            .subcommand(
                SubCommand::with_name("info")
                    .about("Shows general information about your account")
            )
            .subcommand(
                SubCommand::with_name("host")
                    .about("View all available information for an IP address")
                    .arg(
                        Arg::with_name("ip")
                            .index(1)
                            .help(" View all available information for an IP address")
                    )
                    .arg(
                        Arg::with_name("output")
                            .short("o")
                            .long("output")
                            .help("Save the host information in the given file (append if file exists).")
                            .value_name("FILENAME")
                    )
                    .arg(
                        Arg::with_name("size")
                            .long("size")
                            .value_name("NUMBER")
                            .help("The size of the number of responses, up to a maximum of 100 (Default 10).")
                    )
                    .arg(
                        Arg::with_name("start")
                            .long("start")
                            .value_name("NUMBER")
                            .help("Starting position of the query (Default 0).")
                    )
                    .arg(
                        Arg::with_name("type")
                            .short("t")
                            .long("type")
                            .value_name("TYPE")
                            .help("Fields displayed:ip,port,title,country,province,city,owner,time,ssldomain. (Default ip,port)")
                    )
                    .setting(AppSettings::ArgRequiredElseHelp)
            )
            .subcommand(
                SubCommand::with_name("search")
                    .about("Search the Quake database")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .arg(
                        Arg::with_name("query_string")
                            .index(1)
                            .help("Quake Querystring; Example: quake search 'port:80'")
                    )
                    .arg(
                        Arg::with_name("query_file")
                            .short("q")
                            .long("query_file")
                            .help("Quake Querystring file; Example: quake search -q test.txt")
                            .value_name("FILENAME")
                    )
                    .arg(
                        Arg::with_name("time_start")
                            .short("s")
                            .long("start_time")
                            .help("Search start time\r\n\
                            Example: quake search 'port:80' -s 2020-01-01")
                            .value_name("TIME START")
                    )
                    .arg(
                        Arg::with_name("time_end")
                            .short("e")
                            .long("end_time")
                            .help("Search end time\r\n\
                            Example: quake search 'port:80' -e 2020-01-01")
                            .value_name("TIME END")
                    )
                    .arg(
                        Arg::with_name("upload")
                            .short("u")
                            .long("upload")
                            .help("Uploading *.txt files containing only IP addresses, with no more than 1000 IPs.\r\n\
                            Example: quake search -u ips.txt")
                            .value_name("IP File")
                    )
                    .arg(
                        Arg::with_name("output")
                            .short("o")
                            .long("output")
                            .help("Save the host information in the given file (append if file exists).")
                            .value_name("FILENAME")
                    )
                    .arg(
                        Arg::with_name("size")
                            .long("size")
                            .value_name("NUMBER")
                            .help("The size of the number of responses, up to a maximum of 100 (Default 10).")
                    )
                    .arg(
                        Arg::with_name("start")
                            .long("start")
                            .value_name("NUMBER")
                            .help("Starting position of the query (Default 0).")
                    )
                    .arg(
                        Arg::with_name("type")
                            .short("t")
                            .long("type")
                            .value_name("TYPE")
                            .help("Fields displayed:ip,port,title,country,province,city,owner,time,ssldomain,domain. (Default ip,port)")
                    )
                    .arg(
                        Arg::with_name("filter")
                            .short("f")
                            .long("filter")
                            .value_name("TYPE")
                            .help("Filter search results with more regular expressions.\r\n\
                            Example: quake search 'app:\"exchange 2010\"' -t ip,port,title -f \"X-OWA-Version: (.*)\"")
                    )
                    .arg(Arg::with_name("cdn")
                        .short("c")
                        .long("cdn")
                        .value_name("NUMBER")
                        .help("Exclude cdn data when parameter is 1,Not excluded by default"))
                    .arg(Arg::with_name("honey_jar")
                        .short("m")
                        .long("honey_jar")
                        .value_name("NUMBER")
                        .help("Exclude honey_jar data when parameter is 1,Not excluded by default"))
                    .arg(Arg::with_name("latest_data")
                        .short("l")
                        .long("latest_data")
                        .value_name("NUMBER")
                        .help("Display latest data when parameter is 1,Not up to date by default"))
                    .arg(Arg::with_name("filter_request")
                        .short("r")
                        .long("filter_request")
                        .value_name("NUMBER")
                        .help("When the parameter is 1, invalid requests are filtered, such as 400, 401, 403 and other request data, the default is not filtered"))
                    .arg(Arg::with_name("deduplication")
                        .short("d")
                        .long("deduplication")
                        .value_name("NUMBER")
                        .help("When the parameter is 1, data deduplication is performed, and no deduplication is performed by default."))
            )
            .subcommand(
                SubCommand::with_name("domain")
                    .about("View all available information for a domain.")
                    .arg(Arg::with_name("cdn")
                        .short("n")
                        .long("cdn")
                        .value_name("NUMBER")
                        .help("Exclude cdn data when parameter is 1,Not excluded by default"))
                    .arg(Arg::with_name("honey_jar")
                        .short("m")
                        .long("honey_jar")
                        .value_name("NUMBER")
                        .help("Exclude honey_jar data when parameter is 1,Not excluded by default"))
                    .arg(Arg::with_name("latest_data")
                        .short("l")
                        .long("latest_data")
                        .value_name("NUMBER")
                        .help("Display latest data when parameter is 1,Not up to date by default"))

                    .arg(
                        Arg::with_name("domain_name")
                            .index(1)
                            .value_name("DOMAIN_NAME")
                            .help("The domain name to be queried.")
                    )
                    .arg(
                        Arg::with_name("count")
                            .short("c")
                            .long("count")
                            .help("Count of results")
                    )
                    .arg(
                        Arg::with_name("size")
                            .long("size")
                            .value_name("NUMBER")
                            .help("The size of the number of responses, up to a maximum of 100 (Default 10).")
                    )
                    .arg(
                        Arg::with_name("start")
                            .long("start")
                            .value_name("NUMBER")
                            .help("Starting position of the query (Default 0).")
                    )
                    .arg(
                        Arg::with_name("output")
                            .short("o")
                            .long("output")
                            .value_name("FILENAME")
                            .help("Output result to file.")
                    )
                    .arg(
                        Arg::with_name("type")
                            .short("t")
                            .long("type")
                            .value_name("TYPE")
                            .help("Fields displayed:domain,ip,port,title. (Default domain, ip, port)")
                    )
                    .arg(Arg::with_name("filter_request")
                        .short("r")
                        .long("filter_request")
                        .value_name("NUMBER")
                        .help("When the parameter is 1, invalid requests are filtered, such as 400, 401, 403 and other request data, the default is not filtered"))
                    .arg(Arg::with_name("deduplication")
                        .short("d")
                        .long("deduplication")
                        .value_name("NUMBER")
                        .help("When the parameter is 1, data deduplication is performed, and no deduplication is performed by default."))
                    .setting(AppSettings::ArgRequiredElseHelp)
            )
            .subcommand(
                SubCommand::with_name("honeypot")
                    .about("Check whether the IP is a honeypot or not.")
                    .arg(
                        Arg::with_name("ip")
                            .index(1)
                            .value_name("ip")
                            .help("The ip address to be queried.")
                    )
                    .setting(AppSettings::ArgRequiredElseHelp)
            )
            .setting(AppSettings::ArgRequiredElseHelp)
            .get_matches();

        match matches.subcommand() {
            ("init", Some(init_match)) => {
                if let Some(api_key) = init_match.value_of("Api_Key") {
                    ApiKey::init(api_key.to_string());
                }
            }
            ("domain", Some(domain_match)) => {
                let domain = match domain_match.value_of("domain_name") {
                    Some(domain) => domain,
                    None => {
                        Output::error(
                            "Error: You must choose a domain name.\r\nPlease execute -h for help.",
                        );
                        std::process::exit(1);
                    }
                };
                let start: i32 = domain_match
                    .value_of("start")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let size: i32 = domain_match
                    .value_of("size")
                    .unwrap_or("10")
                    .parse()
                    .unwrap_or(10);
                if size > 100 {
                    Output::warning("Warning: Size is set to a maximum of 100, if set too high it may cause abnormal slowdowns or timeouts.");
                }
                let query = &format!("domain:*.{}", domain);
                let data_type = domain_match
                    .value_of("type")
                    .unwrap_or("ip,port,domain")
                    .split(",")
                    .collect::<Vec<&str>>();
                let cdn: i32 = domain_match
                    .value_of("cdn")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let mg: i32 = domain_match
                    .value_of("honey_jar")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let zxsj: i32 = domain_match
                    .value_of("latest_data")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let wxqq: i32 = domain_match
                    .value_of("filter_request")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let sjqc: i32 = domain_match
                    .value_of("deduplication")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let response =
                    Quake::query(query, "", start, size, "", "", cdn, mg, zxsj, wxqq, sjqc);

                let output = match domain_match.value_of("output") {
                    Some(name) => name,
                    None => {
                        Quake::show_domain(
                            response,
                            domain_match.is_present("count"),
                            true,
                            data_type,
                        );
                        std::process::exit(0);
                    }
                };
                // save to file.
                match Quake::save_domain_data(output, response, data_type) {
                    Ok(count) => {
                        Output::success(&format!(
                            "Successfully saved {} pieces of data to {}",
                            count, output
                        ));
                    }
                    Err(e) => {
                        Output::error(&format!("Data saving failure:{}", e.to_string()));
                    }
                };
            }
            ("host", Some(host_match)) => {
                let ip = match host_match.value_of("ip") {
                    Some(ip) => ip,
                    None => {
                        Output::error(
                            "Error: You must choose a ip or cidr.\r\nPlease execute -h for help.",
                        );
                        std::process::exit(1);
                    }
                };
                let start: i32 = host_match
                    .value_of("start")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let size: i32 = host_match
                    .value_of("size")
                    .unwrap_or("10")
                    .parse()
                    .unwrap_or(10);
                if size > 100 {
                    Output::warning("Warning: Size is set to a maximum of 100, if set too high it may cause abnormal slowdowns or timeouts.");
                }
                let query = &format!("ip:{}", ip);
                let response = Quake::query_host(query, start, size);
                let output = match host_match.value_of("output") {
                    Some(name) => name,
                    None => {
                        Quake::show_host(response, true);
                        std::process::exit(0);
                    }
                };
                // save to file.
                match Quake::save_host_data(output, response) {
                    Ok(count) => {
                        Output::success(&format!(
                            "Successfully saved {} pieces of data to {}",
                            count, output
                        ));
                    }
                    Err(e) => {
                        Output::error(&format!("Data saving failure:{}", e.to_string()));
                    }
                };
            }
            ("search", Some(search_match)) => {
                let upload = match search_match.value_of("upload") {
                    Some(file_name) => file_name,
                    None => "",
                };
                let query_file = match search_match.value_of("query_file") {
                    Some(query_file) => query_file,
                    None => "",
                };
                let query_string;
                let query = match search_match.value_of("query_string") {
                    Some(query) => query,
                    None => {
                        if upload == "" && query_file == ""{
                            Output::error("Error: You must enter a search syntax.\r\nPlease execute -h for help.");
                            std::process::exit(1);
                        }else if query_file != ""{
                            query_string = Quake::read_file_search(query_file);
                            query_string.as_str()
                        }else {
                            ""
                        }
                    }
                };
                let start: i32 = search_match
                    .value_of("start")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let size: i32 = search_match
                    .value_of("size")
                    .unwrap_or("10")
                    .parse()
                    .unwrap_or(10);
                let cdn: i32 = search_match
                    .value_of("cdn")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let mg: i32 = search_match
                    .value_of("honey_jar")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let zxsj: i32 = search_match
                    .value_of("latest_data")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let wxqq: i32 = search_match
                    .value_of("filter_request")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let sjqc: i32 = search_match
                    .value_of("deduplication")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let time_start = search_match.value_of("time_start").unwrap_or("");
                let time_end = search_match.value_of("time_end").unwrap_or("");
                if size > 100 {
                    Output::warning("Warning: Size is set to a maximum of 100, if set too high it may cause abnormal slowdowns or timeouts.");
                }
                let data_type = search_match
                    .value_of("type")
                    .unwrap_or("ip,port")
                    .split(",")
                    .collect::<Vec<&str>>();
                let filter = search_match.value_of("filter").unwrap_or("");
                if query_file == "" {
                    let response = Quake::query(
                        query, upload, start, size, time_start, time_end, cdn, mg, zxsj, wxqq, sjqc,
                    );
                    let output = match search_match.value_of("output") {
                        Some(name) => name,
                        None => {
                            Quake::show(response, true, filter, data_type);
                            std::process::exit(0);
                        }
                    };
                    // save to file.
                    match Quake::save_search_data(output, response, filter, data_type) {
                        Ok(count) => {
                            Output::success(&format!(
                                "Successfully saved {} pieces of data to {}",
                                count, output
                            ));
                        }
                        Err(e) => {
                            Output::error(&format!("Data saving failure:{}", e.to_string()));
                        }
                    };
                }else {
                    if query != "" {
                        Output::info(&format!("Search with {}", query));
                    }
                    let response = Quake::query_for_scroll(query, size, time_start, time_end, cdn, mg, zxsj, wxqq, sjqc);
                    // Quake::show_scroll(response,true,filter, data_type);
                    let output = match search_match.value_of("output") {
                        Some(name) => name,
                        None=>{
                            Quake::show_scroll(response,true,filter, data_type);
                            std::process::exit(0);
                        }
                    };
                    // save to file.
                    match Quake::save_scroll_data(output, response, filter, data_type) {
                        Ok(count) => {
                            Output::success(&format!(
                                "Successfully saved {} pieces of data to {}",
                                count, output
                            ));
                        }
                        Err(e) => {
                            Output::error(&format!("Data saving failure:{}", e.to_string()));
                        }
                    };
                }
            }
            ("info", Some(_)) => {
                Quake::show_info();
            }
            ("honeypot", Some(honeypot_match)) => {
                let ip = match honeypot_match.value_of("ip") {
                    Some(query) => query,
                    None => {
                        Output::error(
                            "Error: You must choose a ip.\r\nPlease execute -h for help.",
                        );
                        std::process::exit(1);
                    }
                };
                Quake::honeypot(ip.to_string());
            }
            _ => {}
        }
    }
}

pub struct Output;

impl Output {
    pub fn error(msg: &str) {
        println!("{} {}", Red.bold().paint("[!]"), msg);
    }
    pub fn info(msg: &str) {
        println!("{} {}", Blue.bold().paint("[+]"), msg);
    }
    pub fn success(msg: &str) {
        println!("{} {}", Green.bold().paint("[+]"), msg);
    }
    pub fn warning(msg: &str) {
        println!("{} {}", Yellow.bold().paint("[-]"), msg);
    }
}
