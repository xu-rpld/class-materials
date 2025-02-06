fn main() {
    println!("This old man, he played one,\nBut he needs to play more!!");

    // create an array for numbers in line one
    let number_array = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];

    // create an array for numbers in line two
    let lyric_array = ["on my thumb", "on my shoe", "on my knee", "on my door", "on my hive", "on my sticks", "up in heaven", "on my gate", "on my spine", "once again"];

    // loops to create song

    for num in 0..10 {
            println!();
            println!("This old man, he played {},", number_array[num]);
            println!("He played knick-knack {};", lyric_array[num]);
            println!("With a knick-knack paddywack,");
            println!("Give the dog a bone,");
            println!("This old man came rolling home.");
            println!();
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