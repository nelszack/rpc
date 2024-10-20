use std::arch::x86_64::_rdrand16_step;
unsafe fn  comp_choice(compu_choice: &mut i8) {
    let mut something:u16=0  ;
    while !vec![1,2,3,4].contains(&something){
        _rdrand16_step(&mut something);
        *compu_choice = something as i8;
        
    }
    match compu_choice {
        1 => println!("the computer chose rock"),
        2 => println!("the computer chose paper"),
        3 => println!("the computer chose scissors"),
        4 => println!("the computer chose done"),
        _ => println!("computer chose a number that it shouldnt have"),
    }
}

fn yr_choice(your_choice: &mut i8) {
    println!("choose from one of the options:\n1. rock\n2. paper\n3. scissors\n4. quit\nchoose one:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i8 = input.trim().parse().unwrap();
    *your_choice = num;
    match your_choice {
        1 => println!("you chose rock"),
        2 => println!("you chose paper"),
        3 => println!("you chose scissors"),
        4 => println!("goodbye"),
        _ => println!("not a choice"),
    }
}

fn winner(compu_choice: i8, your_choice: i8,win:&mut i32,lose: &mut i32,tie:&mut i32,total:&mut i32,mwin:&mut i32) {
    if your_choice == compu_choice {
        println!("You tied. Try again\n");
        *tie+=1
    } else if your_choice == 1 && compu_choice == 3 {
        println!("You win\n");
        *win+=1
    } else if your_choice == 2 && compu_choice == 1 {
        println!("You win\n");
        *win+=1
    } else if your_choice == 3 && compu_choice == 2 {
        println!("You win\n");
        *win+=1
    } else if compu_choice==4 {
        println!("computer gave up, you win?\n");
        *mwin+=1
    } else {
        println!("You lost\n");
        *lose+=1
    }
    *total+=1;

}

struct Choices{
    user:i8,
    comp:i8
}
struct Gamestats{
    win:i32,
    lose:i32,
    tie:i32,
    total:i32,
    mwin:i32
}
fn main() {
    let mut game=Choices{user:0,comp:0};
    let mut stats=Gamestats{win:0,lose:0,tie:0,total:0,mwin:0};
    let mut specgames=vec![];
    while game.user != 4 {
        yr_choice(&mut game.user);
        if vec![1, 2, 3].contains(&game.user) {
            unsafe {
            comp_choice(&mut game.comp);
            }
            winner(game.comp, game.user,&mut stats.win ,&mut stats.lose,&mut stats.tie,&mut stats.total,&mut stats.mwin);
            specgames.push((game.user,game.comp));
        }   
    }
    let statistics= match stats.mwin>0 {
        true=>format!("stats:\nwin: {0}(+{1}?)/{2} \tlose: {3}/{2}\ttie: {4}/{2}",stats.win,stats.mwin,stats.total,stats.lose,stats.tie),
        _=> format!("stats:\nwin: {0}/{1} \tlose: {2}/{1}\ttie: {3}/{1}",stats.win,stats.total,stats.lose,stats.tie)
    };
    println!("{statistics}");
    let mut numb=0;
    for i in specgames{
        numb+=1;
        let yrchoise=match i.0{
            1=>"rock",
            2=>"paper",
            3=>"scissors",
            4=>"done",
            _=>"unkown"
        };
        let compchoise=match i.1{
            1=>"rock",
            2=>"paper",
            3=>"scissors",
            4=>"done",
            _=>"unkown"
        };
        println!("{}. your choise {}, computer choise {}",numb,yrchoise,compchoise)
    }
}
