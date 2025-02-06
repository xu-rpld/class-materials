fn main() {
    println!("This old man song:\n");
    
    //Array holding the 'actions' by the old man on the certain verse.
    let actions = [
        "on my thumb", 
        "on my shoe", 
        "on my knee", 
        "on my door",
        "on my hive", 
        "on my sticks", 
        "up in heaven", 
        "on my gate",
        "on my spine", 
        "once again"
    ];
    
    // Verse containing associated number to help with corresponding action.
    let verse = [
        "one", 
        "two", 
        "three", 
        "four", 
        "five", 
        "six", 
        "seven", 
        "eight", 
        "nine", 
        "ten"
    ];
    
    // Loop that goes through the length of the actions and increments the number.
    for i in 0..actions.len() {
        println!("This old man, he played {},", verse[i]);
        println!("He played knick-knack {};", actions[i]);
        println!("With a knick-knack paddywhack,");
        println!("Give the dog a bone,");
        println!("This old man came rolling home.\n");
        // Break for continuity.
    }
}


/* Lyrics to This Old Man

This old man, he played one,
He played knick-knack on my thumb;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played two,
He played knick-knack on my shoe;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played three,
He played knick-knack on my knee;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played four,
He played knick-knack on my door;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played five,
He played knick-knack on my hive;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played six,
He played knick-knack on my sticks;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played seven,
He played knick-knack up in heaven;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played eight,
He played knick-knack on my gate;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played nine,
He played knick-knack on my spine;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played ten,
He played knick-knack once again;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.
*/