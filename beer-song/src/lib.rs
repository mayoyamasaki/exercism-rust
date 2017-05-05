pub fn verse(n: i64) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
         Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\n\
         Take it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else if n == 2 {
        "2 bottles of beer on the wall, 2 bottles of beer.\n\
         Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
    } else {
        format!("{} bottles of beer on the wall, {} bottles of beer.\n\
                 Take one down and pass it around, {} bottles of beer on the wall.\n",
                 n, n, n-1)
    }
}


pub fn sing(to: i64, from: i64) -> String {
    let song = (from+1..to+1)
               .rev()
               .fold("".to_string(),
                     |sum, n| sum + &verse(n) + "\n");
    song + &verse(from)
}
