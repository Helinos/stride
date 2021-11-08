use std::collections::VecDeque;

use serenity::{
    model::channel::Message, 
    Result as SerenityResult,
};


// Checks that a message was successfully sent; if not, then logs why to stdout.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}



pub async fn seconds_to_string(secs: u64) -> String {
    // if the duration is greater than an hour we need a different string
    if secs >= 3600 {
        let s = secs % 60;
        let m = (secs / 60) % 60;
        let h = (secs / 60) / 60;
        return format!("{}:{:0>2}:{:0>2}", h, m, s);
    } else {
        let s = secs % 60;
        let m = (secs / 60) % 60;
        return format!("{}:{:0>2}", m, s);
    }
}



const REPLACEMENTS: [char; 7] = ['［', '］','❨', '❩', '˂', '˃', 'ˋ'];

pub async fn clean_title(mut title: String) -> String {
    if title.len() > 64 {
        title.truncate(64);
        title += "...";
    }

    for (index, char) in "[]()<>`".chars().enumerate() {
        if title.contains(char) {
            title = title.replace(char, format!("\\{}", REPLACEMENTS[index]).as_str());
        }
    }

    title
}



// Real requirement for shuffle
pub trait LenAndSwap {
    fn len(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
}

// An exact copy of rand::Rng::shuffle, with the signature modified to
// accept any type that implements LenAndSwap
pub fn shuffle<T, R>(values: &mut T, mut rng: R)
where
    T: LenAndSwap,
    R: rand::Rng,
{
    let mut i = values.len();
    while i >= 2 {
        // invariant: elements with index >= i have been locked in place.
        i -= 1;
        // lock element i in place.
        values.swap(i, rng.gen_range(0..i + 1));
    }
}

// VecDeque trivially fulfills the LenAndSwap requirement, but
// we have to spell it out.
impl<T> LenAndSwap for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.swap(i, j)
    }
}
