fn main() {
    //array holding the things that change between verses
    let verses: [(&str, &str); 10] = [ 
        ("one", "on my thumb"), 
        ("two", "on my shoe"), 
        ("three", "on my knee"), 
        ("four", "on my door"),
        ("five", "on my hive"),
        ("six", "on my sticks"),
        ("seven", "up in heaven"),
        ("eight", "on my gate"),
        ("nine", "on my spine"),
        ("ten", "once again"),
    ];

    //loop running through the framework of the verses
    for x in 0..10 {
        println!("This old man, he played {},
        He played knick-knack {};
        With a knick-knack paddywhack,
        Give the dog a bone,
        This old man came rolling home.", verses[x].0, verses[x].1);
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