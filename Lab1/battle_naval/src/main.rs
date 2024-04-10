mod battle_naval;
use clap::{Arg, Command};

fn boat_value_parser(s: &str) -> Result<(usize, char),String>{
    if let Some(c) = s.chars().last(){
        if c == 'V' || c == 'H'{
            if s.chars().nth(0).is_some() && s.chars().nth(0).unwrap().to_digit(10).is_some() && (1..=4).contains(&s.chars().nth(0).unwrap().to_digit(10).unwrap()){
                return Ok((s.chars().nth(0).unwrap().to_digit(10).unwrap() as usize, c));
            }
        }
    }
    return Err("\x1b[31mValore errato!\x1b[0m. Per favore inserisci un valore tra 1 e 4 seguito da V o H.".to_string());
}

fn boats_value_parser(s: &str) -> Result<[u8; 4],String>{
    let mut boats: [u8; 4] = [0; 4];
    if s.split(',').count() == 4{
        for (idx,s) in s.split(',').enumerate(){
            if let Ok(value) = s.parse::<u8>(){
                boats[idx] = value;
            }else{
                return Err("\x1b[31mValori errati!\x1b[0m Devi inserire 4 valori u8 separati da una virgola.".to_string());
            }
        }
        return Ok(boats);
    }else{
        return Err("\x1b[31mValori errati!\x1b[0m Devi inserire 4 valori u8 separati da una virgola.".to_string());
    }
}

fn start_value_parser(s: &str) -> Result<(usize, usize),String>{
    if s.split(',').count() == 2{
        if s.split(',').nth(0).is_some() && s.split(',').nth(0).unwrap().parse::<usize>().is_ok() && s.split(',').nth(1).is_some() && s.split(',').nth(1).unwrap().parse::<usize>().is_ok(){
            return Ok((s.split(',').nth(0).unwrap().parse::<usize>().unwrap(), s.split(',').nth(1).unwrap().parse::<usize>().unwrap()));
        }
    }
    return Err("\x1b[31mValori errati!\x1b[0m. Per favore inserisci due u8 separati da virgola.".to_string());
    
}

fn main(){
    let matches = Command::new("BattleNaval")
        .version("1.0")
        .author("Vitabile")
        .about("The creation of a battle naval board.")
        .next_line_help(true)
        .arg(Arg::new("mode").value_parser(["new","add"]))
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("file.txt")
                .required(true)
        )
        .arg(
            Arg::new("boat")
                .short('b')
                .long("boat")
                .value_name("valueV/H")
                .required_if_eq("mode", "add")   
                .value_parser(boat_value_parser)
        )
        .arg(
            Arg::new("start")
                .short('s')
                .long("start")
                .value_name("x,y")
                .required_if_eq("mode", "add")
                .value_parser(start_value_parser)
                
        )
        .arg(
            Arg::new("boats")
                .long("boats")
                .value_name("N1,N2,N3,N4")
                .required_if_eq("mode", "new")
                .value_parser(boats_value_parser)
        )
        .get_matches();
    
    // safe unwrap checked by parser if is a correct value
    let mode = matches.get_one::<String>("mode").unwrap();
    let file_path = matches.get_one::<String>("file").unwrap();

    match mode.as_str() {
        "new" => {
            let boats = matches.get_one::<[u8; 4]>("boats").unwrap();
            battle_naval::write_board(file_path,&mut battle_naval::Board::new(boats));
        }
        "add" => {
            let (dim,orientation) = *matches.get_one::<(usize, char)>("boat").unwrap();
            let pos = *matches.get_one::<(usize, usize)>("start").unwrap();
            let boat: battle_naval::Boat;
            if orientation == 'V'{
                boat = battle_naval::Boat::Vertical(dim);
            }else{
                boat = battle_naval::Boat::Horizontal(dim);
            }
            let res = battle_naval::read_board(file_path);
            match res {
                Ok(mut board) => {battle_naval::update_board(&mut board, boat, pos, file_path);},
                Err(e) => {println!("{}",e);}
            }
        }
        _ => {println!("Errore Impossibile");}
    }


}