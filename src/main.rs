use std::io;
use std::collections::HashMap;

fn main() {
    //Create score and door numbers
    let mut score = 0;
    let mut door_number = 0;

    println!("***Intro***");
    intro(&mut score, &mut door_number);

    //Choose door
    if door_number == 1 {
        println!("***Door 1***");
        door_1(&mut score);
    } else if door_number == 2 {
        println!("***Door 2***");
        door_2(&mut score);
    } else {
        return
    }

    the_end(&mut score)
}

fn intro(score:&mut i32, door_number:&mut i32) {
    println!("Hello, what is your name?");

    // Take user input for variable 'name'
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to readline");

    println!("Welcome {name} to the Rusty Treasure Hunt! Your goal will be to collect as much treasure
    as possible and make it out with everything you've collected. Choose your path wisely or all of
    your talent will go to waste.");
    println!("As a welcoming gift, I'll give you a good start. Enter the front door to my 
    mansion and you'll see two doors to choose from. Any of them will lead you
    forward, but be careful. The further you go, the riskier it gets!");

    //Prompt user to continue game
    println!("Do you wish to continue?");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("failed to readline");
    if answer.trim() == "yes" {
        println!("You enter the house and see the three doors before you.");
        *score += 1;
    } else if answer.trim() == "Yes" {
        println!("You enter the house and see the three doors before you.");
        *score += 1;
    } else {
        println!("Oh well, it's a little late to decide that. You are drawn into the house.");
        *score -= 1;
    }
    println!("Which door will you choose? (1 or 2)");

    //Prompt user to choose a door number
    let mut door = String::new();
    io::stdin().read_line(&mut door).expect("failed to readline");
    let door_choice: i32 = door.trim().parse().expect("please give me correct string number!");
    *door_number = door_choice;
}

fn door_1(score:&mut i32) {
    let mut hunting = 10;
    let gold = 5;
    println!("You have chosen door #1. I do hope this ends well for you. You'll
    have several chances at gaining gold, but be careful, you never know when your time
    will run out. You enter a dark and gloomy room, only a pale light illuminating a small
    table in the center of the room. On the table you see strange wooden panels with the
    numbers 0 - 9 numbered on them. You feel that you can press them like buttons.");

    //Prompt user to choose any number and score if number is even
    while hunting > 0 {
        println!("Which button(s) do you press?");
        let mut button = String::new();
        io::stdin().read_line(&mut button).expect("failed to readline");
        let button_number: i32 = button.trim().parse().expect("please give me correct string number!");
        if button_number % 2 == 0 {
            println!("A secret compartment in the wall nearby opens up and you find a gold coin!");
            *score += gold;
        } else {
            println!("Sorry to say you didn't choose wisely and you feel your pockets feel ligher
            than they did before.");
            *score -= 3;
        }
        hunting -= 1;
    }
    println!("Time is up! I hope you filled your pockets full of treasure");
}

fn door_2(score:&mut i32) {
    //Intro to door 2
    println!("You have chosen door #2. As you enter you're amazed at a sea of random items scattered across
    a room you can't quite see the end of. From the back of the room you hear a deep rumble, what might even
    be a growl. In fear you realize you should grab what you can and get out. Before you there are several items
    that stand out but you have only time to collect three. Which of the treasures do you choose to grab?");
    println!("Old coin, Scroll, Watch, Knife, Unusual stone, Ring, Candlestick, Revolver, Small picture, and Handheld mirror");
    
    //Create Hashmap of items
    let mut contacts = HashMap::new();
    contacts.insert("Old coin", "2");
    contacts.insert("Scroll", "1");
    contacts.insert("Watch", "3");
    contacts.insert("Knife", "2");
    contacts.insert("Unusual stone", "4");
    contacts.insert("Ring", "3");
    contacts.insert("Candlestick", "2");
    contacts.insert("Revolver", "4");
    contacts.insert("Small picture", "1");
    contacts.insert("Handheld mirror", "2");

    println!("Choose one item at a time.");
    let mut counter = 3;

    //Loop through and ask user for three items, adjusting score as needed.
    while counter > 0 {
        println!("What item do you choose?");
        let mut choices = String::new();
        io::stdin().read_line(&mut choices).expect("failed to readline");
        match choices.trim() {
            "Old coin" => *score += 2,
            "Scroll" => *score += 1,
            "Watch" => *score += 3,
            "Knife" => *score += 2,
            "Unusual stone" => *score += 4,
            "Ring" => *score += 3,
            "Candlestick" => *score += 2,
            "Revolver" => *score += 4,
            "Small picture" => *score += 1,
            "Handheld mirror" => *score += 2,
            &_ => *score += 1,
        }
        counter -= 1;
    }
    println!("After collecting your treasures you book it out of the room. Once safe you
    take the time to inspect your treasures.");
    println!("Time to add up your treasures and see what they were worth!");
}

// Show the end score
fn the_end(score:&mut i32) {
    println!("Congratulations! You've collected: {score} Gold Coins. Please come again!");
}