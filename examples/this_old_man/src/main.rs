fn main() {
    let first_line = "This old man, he played ";
    let second_line = "He played knick-knack on my ";
    let rest = "With a knick-knack paddywhack,\nGive the dog a bone,\nThis old man came rolling home.";
    let mut i = 0;
    let numbers = ["one", "two", "three", "four", "five", 
        "six", "seven", "eight", "nine"];
    let locations = ["thumb", "shoe", "knee", "door",
        "hive", "sticks", "heaven", "gate", "spine"];

    while i < 9 {
        let mut val = first_line.to_owned();
        let number = numbers[i];
        val.push_str(number);
        println!("{val},");

        val = second_line.to_owned();
        let location = locations[i];
        val.push_str(location);
        println!("{val};");
        
        println!("{rest}\n");
        i = i + 1;
    }
    println!("This old man, he played ten");
    println!("He played knick-knack once again");
    println!("{rest}");
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