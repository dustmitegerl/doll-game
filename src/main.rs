extern crate rand;
use rand::{random_range};
use std::io::{self, stdin, stdout, Read, Write, Result};
use std::time::Duration;
use std::thread::{sleep, spawn};

// mechanics

fn scroll(s: &str) {
    let output = "\n".to_owned() + s + "\n";
    for c in output.chars() {
        print!("{c}");
        stdout().flush().unwrap();
        sleep(Duration::from_millis(10));
    }
    sleep(Duration::from_millis(500));
}

fn pause(action: &str) {
    let continue_message = "[Press Enter to ".to_owned() + action + "...]";
    scroll(&continue_message);
    let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
}


// sections

fn intro() {
    scroll("content warning: mild violence/gore. ");
    pause("continue");
    scroll("welcome!");
    scroll("you won\'t remember this, but--");
    scroll("well,");
    scroll("you, silly spirit, were flying through the air\n\
        on your way Out, and got distracted");
    scroll("by a dark house\n\
        in the dark woods");
    scroll("as your illusory heart drew you to something forgotten\n\
        and you, a creature without memory,");
    scroll("entombed yourself in a doll.");
}

fn unfinished() {
    scroll("sorry, this path is under construction. please choose another.");
}

fn wrong() {
    scroll("silly doll! please choose from the given options!\n\
    letter for letter, dear. case sensitive.");
}

fn fight_outcome(choice: &str) {
    let outcomes = vec!["1", "sleepy", "eaten"];
    let outcome = outcomes[random_range(0..outcomes.len())];
    
    match outcome {
        "1" => {
            if choice == "fightBack" {
                fight();
            } else if choice == "giveUp" {
                give_up();
            }
        },
        "sleepy" => sleepy(),
        "eaten" => eaten(),
        _ => {}
    }
}

fn fight() {
    let flips = random_range(0..13);
    let mut flip = 0;
    
    while flip < flips {
        if flip % 2 == 0 {
            println!("doll is on top!");
        } else {
            println!("null is on top!");
        }
        flip += 1;
    }
    
    scroll("obviously, null once again has doll pinned. its eyes reveal an obvious, growing hunger.\n");
    scroll("does doll keep fighting? y/n");
    
    let mut keep_fighting = String::new();
    stdin().read_line(&mut keep_fighting).unwrap();
    let keep_fighting = keep_fighting.trim();
    
    match keep_fighting {
        "y" => fight_outcome("fightBack"),
        "n" => fight_outcome("giveUp"),
        _ => {
            wrong();
            fight();
        }
    }
}

fn give_up_choice() {
    scroll("would it like to try again? y/n");
    
    let mut try_again = String::new();
    stdin().read_line(&mut try_again).unwrap();
    let try_again = try_again.trim();
    
    match try_again {
        "y" => fight(),
        "n" => sleepy(),
        _ => {
            wrong();
            give_up_choice();
        }
    }
}

fn give_up() {
    scroll("probably for the best! nothing disappointing about a doll giving up.\n\
         null sees the submission in doll's eye and rolls off, shifting its lock into a hug\n\
         should doll try to take advantage, both of them know it would fail.\n\
         after a moment of rest, null asks doll if it would like to try again.");
    give_up_choice();
}

fn end() {
    scroll("the end. wake up? y/n");
    stdout().flush().unwrap();
    
    let mut restart = String::new();
    stdin().read_line(&mut restart).unwrap();
    let restart = restart.trim();
    
    match restart {
        "y" => start(),
        "n" => println!("thanks for giving the little doll another day of its story."),
        _ => {
            wrong();
            end();
        }
    }
}

fn sleepy() {
    scroll("doll feels sleep suddenly weigh over, somewhat blissfully, somewhat sadly awaiting\n\
    a rest deeper than any animal\'s somnolence. a final thought, something like a dream\n\
    slugs through the doll\'s ersatz thoughts like heavy, dusty vapor\n\
    before it is returned to the Stillness.");
    end();
}

fn wrestle_choice() {
    println!("does doll struggle? y/n");
    stdout().flush().unwrap();
    
    let mut struggle = String::new();
    stdin().read_line(&mut struggle).unwrap();
    let struggle = struggle.trim();
    
    match struggle {
        "y" => fight(),
        "n" => give_up(),
        _ => {
            wrong();
            wrestle_choice();
        }
    }
}

fn wrestle() {
    scroll("though doll is pretty weak, its long limbs and loose joints may serve an advantage \n\
        against null\'s superior strength");
    scroll("...well, almost -- but it doesn\'t take long before null\'s got doll caught in her trap!");
    wrestle_choice();
}

fn coffee() {
    scroll("ooh, toasty choice!");
    scroll("excited for the caffeine hype, doll and null decide to wrestle in the rainy courtyard.");
    wrestle();
}

fn fight_choice() {
    println!("would doll like to wrestle its friend null? y/n");
    stdout().flush().unwrap();
    
    let mut fight_or_not = String::new();
    stdin().read_line(&mut fight_or_not).unwrap();
    let fight_or_not = fight_or_not.trim();
    
    match fight_or_not {
        "y" => wrestle(),
        "n" => {
            scroll("doll decides not, and after a while, decides to read alone for a while\n\
                before moving on with its day.");
            start();
        }
        _ => {
            wrong();
            fight_choice();
        }
    }
}

fn lapsang() {
    scroll("doll locks eyes with never, and they nod together, a little gleam matched between them. \n\
        the coffee must have really cranked null, because suddenly it asks doll if it would like \n\
        to go wrestle in the parlor.");
    fight_choice();
}

fn valerian() {
    scroll("knowing what\'s to come, nix excitedly though already sedatedly lifts dolls hand\n\
        and guides it to parlor, to the fireplace. each takes one of the couches, angled\n\
        slightly, side by side. their fingers still ever so slightly clasped, they gaze\n\
        into the fire, occasionally glancing over to one another, syncing sleepiness\n\
        with each other\'s eyes.\n");
    sleepy();
}

fn plum() {
    scroll("after cozing with friends awhile, doll decides to get some alone time to read\n\
        from the house\'s immense library for a while, learning about dragons\n\
        and artificial biochemistry. after that, it practices drawing, creating pastel\n\
        pictures of various objects in the common spaces as well as in its friends\' rooms.");
    sleepy();
}

fn tea_choice() {
    scroll("the house has \'coffee\', \'lapsang\', \'valerian\' and \'plum\'.");
    stdout().flush().unwrap();
    
    let mut tea = String::new();
    stdin().read_line(&mut tea).unwrap();
    let tea = tea.trim();
    
    match tea {
        "coffee" => coffee(),
        "lapsang" => lapsang(),
        "valerian" => valerian(),
        "plum" => plum(),
        _ => {
            wrong();
            tea_choice();
        }
    }
}

fn tea_party() {
    scroll("the doll wants to have a tea party. how cute :>\n\
        so here it is, with a few friends -- null, nix and never, the triplets.\n\
        null wants coffee (yes, there is coffee); nix wants valerian; never wants lapsang.");
    scroll("so, what would doll like?");
    tea_choice();
}

fn clean() {
    scroll("doll spends all day sweeping, mopping, wiping, scrubbing, shining, kissing and dusting.");
    sleepy();
}

fn eaten() {
    scroll("the monster (after all, who does doll know that isn\'t a monster?)\n\
        pins the weary, broken doll\n\
        to the floor with little effort. doll feels its nerve-substance leaking from its broken joints\n\
        and looks pleadingly into the eyes of its opponent, knowing from touch alone what was coming\n\
        and that there is no way out. giving small struggle, it nonetheless signals submission\n\
        with a blink and sees it\'s dear friend, overcome with love, close its jaws over the doll\'s face,\n\
        pulling back to tear it to pieces with claws and sharp teeth.\n\
        already blinded, the last thing doll senses is the total heat of its loving friend blending with\n\
        its torn body rupturing, acrid-sweet polyphenols reacting with caustic saliva, sizzling and frothing\n\
        on its papersoft potash-dusted skin.\n\
        of course, doll thinks, this isn\'t the end -\n\
        doll knows her hungry friends will always return it remains,\n\
        once processed, to their witch so that it can repair it for the next day, or whatever day\n\
        it gets around to it.");
    sleepy();
}

fn start() {
    scroll("so the what does the little doll want to do today?");
    scroll("it can choose \'tea party\', \'clean the house\', or \'get eaten\'.");
    
    let mut activity = String::new();
    stdin().read_line(&mut activity).unwrap();
    let activity = activity.trim();
    
    match activity {
        "tea party" => tea_party(),
        "clean the house" => clean(),
        "get eaten" => eaten(),
        _ => {
            wrong(); start();
        },
    }
}

fn main() {
    intro();
    pause("continue");
    start();
    pause("end game");
}
