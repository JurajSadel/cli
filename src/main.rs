use serde::{Deserialize, Serialize};
use clap::{App, Arg};


#[derive(Deserialize, Debug)]
struct Envelope {
    results: Vec<Chip>
}

#[derive(Deserialize, Serialize, Debug)]
struct Chip
{
    name: String,
    bluetooth: String,
    operatingTemp: String,
    psram: i16

}

//filter by entered chip name as cmd argument
fn parse_name(name_arg:Vec<&str>, raw:&Envelope )
{
    for name in name_arg {
        print!("\nName {}:\n", name);
        
        let mut unknown_chip = true;
        for chip in &raw.results { 
            
            if chip.name == name 
            {
                println!("bluetooth: {}", chip.bluetooth);
                if chip.psram == 0
                {
                    println!("psram: Not present");
                }
                else {
                    
                    println!("psram: {}", chip.psram);
                }

                unknown_chip = false;
                break;
            }
        };

    if unknown_chip == true
    {
        println!("Assigned unknown name!");
    }
}
}

//parse string temperature into integers
fn parse_string(temperature_str:&str) -> (i16, i16)
{
    let v:Vec<&str> = temperature_str.split(|c| c == ' ').collect();
    
    let min_temp = v[0].parse::<i16>().unwrap();
    let mut max_temp = 0;

    if v.len() == 3
    {
         max_temp = v[2].parse::<i16>().unwrap();
    }
    return (min_temp, max_temp)
}

//filter by entered temperature as cmd argument
fn parse_temperature(temperature_arg:&str, raw:&Envelope )
{
    let pair_arg_temp = parse_string(temperature_arg);

    print!("\nTemperature {}:\n[ ",temperature_arg);
    for chip in &raw.results
    {
        let pair_chip_temp = parse_string(&chip.operatingTemp);
        if pair_arg_temp.0 >= pair_chip_temp.0 && pair_arg_temp.0 <= pair_chip_temp.1
        {
            print!("{} ", chip.name);
        }
    }
    print!(" ]");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	
    let matches = App::new("My Super Program")
                            .arg(Arg::with_name("name")
                                .short("n")
                                .long("name")
                                .multiple(true)
                                .takes_value(true)
                                .help("Chip Name"))
                            .arg(Arg::with_name("operatingTemp")
                                .allow_hyphen_values(true)
                                .short("t")
                                .long("temperature")
                                .takes_value(true)
                                .help("Operating Temperature"))
                            .get_matches();

    //let name_arg = matches.value_of("name").unwrap_or("");
    let name_arg:Vec<&str>  = matches.values_of("name").unwrap().collect();
    //let name_arg  = matches.values_of("name").expect("Chip name missing!");

    let temperature_arg = matches.value_of("operatingTemp").unwrap_or("");
    
    //println!("{:?}", name_arg);
    //println!("{:?}", temperature_arg);
    
    //get JSON data
    let response = reqwest::blocking::get("https://products.espressif.com/iot-solution-api/query?language=en")?;
    let text = response.text()?;
    let raw: Envelope = serde_json::from_str(&text)?;
    
    //let filtered = serde_json::to_string(&raw.results)?;
    //println!("{}", &filtered);
    //std::fs::write("data.json", filtered)?;

    parse_name(name_arg, &raw);
    parse_temperature(temperature_arg, &raw);

	Ok(())

}
