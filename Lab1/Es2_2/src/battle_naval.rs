use std::fs;

const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}

impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        let boats: [u8; 4] = <[u8; 4]>::try_from(boats).unwrap();
        let data = [[0; BSIZE]; BSIZE];
        Board{boats, data}
    }
    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String)->Board {
        let mut boats: [u8; 4] = [0; 4];
        let mut data: [[u8; BSIZE]; BSIZE] = [[0; BSIZE]; BSIZE];
        let rows: Vec<&str> = s.split('\n').collect();
        let boats_str: Vec<&str> = rows[0].split(' ').collect();
        for (i,boat_str) in boats_str.iter().enumerate(){
            boats[i] = boat_str.parse().unwrap();
        }
        let mut element_data: [u8; BSIZE] = [0; BSIZE];
        for (i,row) in rows[1..BSIZE].iter().enumerate(){
            for (j,c) in row.chars().enumerate(){
                if c == ' '{
                    element_data[j] = 0;
                }else{
                    element_data[j] = 1;
                }
            }
            data[i] = element_data;
        }
        Board{boats,data}
    }
    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(&mut self, boat: Boat, pos: (usize, usize)) -> Result<(), Error> {
        let len;
        let final_col;
        let final_row;
        match boat{
            Boat::Vertical(v)=> 
            {
                if self.boats[v-1] <= 0{
                    return Err(Error::BoatCount);
                }else if pos.0 + v >= BSIZE {
                    return Err(Error::OutOfBounds);
                }else{
                    final_col = if pos.1 == BSIZE-1 {pos.1} else {pos.1 + 1};
                    final_row = if pos.0 + v - 1 == BSIZE-1 {pos.0 + v - 1} else {pos.0 + v};
                }
                len = v;
            },
            Boat::Horizontal(h) => 
            {
                if self.boats[h-1] <= 0{
                    return Err(Error::BoatCount);
                }else if pos.1 + h >= BSIZE {
                    return Err(Error::OutOfBounds);
                }else{
                    final_col = if pos.1 + h - 1 == BSIZE-1 {pos.1 + h - 1} else {pos.1 + h};
                    final_row = if pos.0 == BSIZE-1 {pos.0} else {pos.0 + 1};
                }
                len = h;
            }
        }
        let start_row = if pos.0 == 0 {pos.0} else {pos.0 - 1};
        let start_col = if pos.1 == 0 {pos.1} else {pos.1 - 1};
        for r in start_row..=final_row{
            for c in start_col..=final_col{
                if self.data[r][c] == 1 {
                    println!("{r},{c}");
                    return Err(Error::Overlap);
                }
            }
        }
        self.boats[len - 1] -= 1;
        match boat{
            Boat::Vertical(v) => for i in 0..v{
               self.data[pos.0 + i][pos.1] = 1; 
            },
            Boat::Horizontal(h) => for i in 0..h{
                self.data[pos.0][pos.1 + i] = 1;
            }
        }
        Ok(())
    }

    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String {
        let mut string = String::from(format!("{} {} {} {}\n", self.boats[0], self.boats[1], self.boats[2], self.boats[3]));
        for row in self.data {
            for col in row {
                if col == 0 {
                    string.push(' ');
                }else{
                    string.push('B');
                }
            }
            string.push_str("\n");
        }
        string
    }    
}

pub fn read_board(path: &str) -> Result<Board,String>{
    let result = fs::read_to_string(path);
    match result {
        Ok(str) => Ok(Board::from(str)),
        Err(_) => Err("Impossibile aggiungere la Boat: \x1b[31mPath non trovato!\x1b[0m".to_string()),
    }
}
pub fn write_board(path: &str, board: &mut Board){
    let res = fs::write(path,board.to_string());
    match res{
        Ok(_) => {println!("\x1b[32mBoard aggiornata!\x1b[0m");},
        Err(e) => {println!("{:?}",e);}
    }
}

pub fn update_board(board: &mut Board, boat: Boat, pos: (usize, usize),path: &str){
    let res = board.add_boat(boat, pos);
    match res{
        Ok(_) => {
            write_board(path, board);
        },
        Err(e) => match e{
            Error::Overlap => println!("Impossibile aggiungere la Boat: \x1b[31mOverlap encountered!\x1b[0m"),
            Error::OutOfBounds => println!("Impossibile aggiungere la Boat: \x1b[31mOutOfBounds encountered!\x1b[0m"),
            Error::BoatCount => println!("Impossibile aggiungere la Boat: \x1b[31mBoatCount encountered!\x1b[0m"),
        }
    }
}