fn main() {
    let things = [
        "thumb", "shoe", "knee", "door", "hive",
        "sticks", "heaven", "gate", "spine", "once again"
    ];

    let spelled_number = [
        "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten"
    ];

    for num in 1..=10 {
        println!("This old man, he played {},", spelled_number[num - 1]);
        println!("He played knick-knack on my {};", things[num-1]);
        println!("With a knick-knack paddywhack,\nGive the dog a bone,\nThis old man came rolling home.\n");
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