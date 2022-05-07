fn contains(mut occupied: [[usize; 2]; 9], new: [usize; 2])->bool{
 let mut exists: bool = false;

 for step in 0..9{
  if occupied[step][0] == new[0] && occupied[step][1] == new[1]{ 
   exists = true; 

   break; 
  } //if occupied[step][0] == new[0] && occupied[step][1] == new[1]{
 } //for step in 0..9{

 exists
} //fn contains(mut occupied: [[usize; 2]; 9], new: [usize; 2])->bool{

fn input(occupied: [[usize; 2]; 9])->[usize; 2]{
 const   empty :  String     = String::new();

 let mut exists:  bool       =  false       ;
 let mut string: [String; 2] = [empty ; 2]  ;
 let mut usize : [usize ; 2] = [0usize; 2]  ;

 loop{
  loop{
   string[0] = empty;
   string[1] = empty;

   println!("x:"); std::io::stdin().read_line(&mut string[0]);
   println!("y:"); std::io::stdin().read_line(&mut string[1]);

   string[0] = string[0].replace(" ", "").replace("\n", "").replace("\r", "");
   string[1] = string[1].replace(" ", "").replace("\n", "").replace("\r", "");

   if string[0].is_empty() || string[1].is_empty(){
    println!("You input empty value! Try again!");

   }else{ //if string[0].is_empty() || string[1].is_empty(){
    if string[0].chars().all(char::is_numeric) && string[1].chars().all(char::is_numeric){
     usize[0] = string[0].parse::<usize>().unwrap();
     usize[1] = string[1].parse::<usize>().unwrap(); 

     if usize[0] > 0 && usize[0] < 4 && usize[1] > 0 && usize[1] < 4{ 
      break;

     }else{ //if usize[0] > 0 && usize[0] < 4 && usize[1] > 0 && usize[1] < 4{
      println!("Your digit must have been from 1 to 3! Try again!");

     } //}else{ //if usize[0] > 0 && usize[0] < 4 && usize[1] > 0 && usize[1] < 4{

    }else{ //if string[0].chars().all(char::is_numeric) && string[1].chars().all(char::is_numeric){
     println!("You must have input only digits! Try again!");

    } //}else{ //if string[0].chars().all(char::is_numeric) && string[1].chars().all(char::is_numeric){
   } //}else{ //if string[0].is_empty() || string[1].is_empty(){
  } //loop{

  exists = contains(occupied, usize);

  if exists {
   println!("This position is already occupied!");

  }else{ //if exists {
   break;

  } //}else{ //if exists {
 } //loop{

 usize
} //fn input(occupied: [[usize; 2]; 9])->[usize; 2]{

fn show(step: [[char; 3]; 3]){
 println!("-------------");
 println!("| {} | {} | {} |", step[0][0], step[0][1], step[0][2]);
 println!("-------------");
 println!("| {} | {} | {} |", step[1][0], step[1][1], step[1][2]);
 println!("-------------");
 println!("| {} | {} | {} |", step[2][0], step[2][1], step[2][2]);
 println!("-------------");
} //fn show(step [[char; 3]; 3]){

fn main(){
 let mut cell    :   char          = ' '              ;
 let mut game    : [[char ; 3]; 3] = [[' '; 3]; 3]    ;
 let mut last    :   usize         = 0                ;
 let mut occupied: [[usize; 2]; 9] = [[0usize; 2 ]; 9];
 let mut usize   : [ usize; 2]     = [0usize; 2]      ;

 show(game);
 
 for step in 0..9{
  if step%2 == 1 { cell = 'x'; }else{ cell = '0'; } 

  usize = input(occupied);

  occupied[last][0] = usize[0]; 
  occupied[last][1] = usize[1];

  usize[0]-=1;
  usize[1]-=1;

  last+=1;

  game[usize[0]][usize[1]] = cell; 

  show(game);
 } //for step in 0..9{
} //fn main(){
