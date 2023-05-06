use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Tweet {
    user_id: i32,
    tweet_id: i32,
}

impl Tweet {
    fn new(user_id: i32, tweet_id: i32) -> Self {
        Self { user_id, tweet_id }
    }
}

#[derive(Debug)]
struct Twitter {
    followers: HashMap<i32, Vec<i32>>,
    tweets: VecDeque<Tweet>
}

impl Twitter {

    fn new() -> Self {
        Self {
            tweets: VecDeque::new(),
            followers: HashMap::new()
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push_front(Tweet::new(user_id, tweet_id));
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let tweets_iter = self.tweets.iter();
        let followers = self.followers.get(&user_id);

        if followers.is_none() {
            return tweets_iter
                .filter(|tweet| tweet.user_id == user_id)
                .take(10)
                .map(|tweet| tweet.tweet_id)
                .collect()
        };

        tweets_iter
            .filter(|tweet| tweet.user_id == user_id || followers.unwrap().contains(&tweet.user_id))
            .take(10)
            .map(|tweet| tweet.tweet_id)
            .collect()
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.followers
            .entry(follower_id)
            .and_modify(|f| f.push(followee_id))
            .or_insert_with(|| vec![followee_id]);
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.followers
            .get_mut(&follower_id)
            .unwrap_or(&mut vec![])
            .retain(|&x| x != followee_id);
    }

}

fn main() {

    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    twitter.get_news_feed(1);
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    twitter.get_news_feed(1);
    twitter.unfollow(1, 2);
    twitter.get_news_feed(1);
    twitter.follow(1, 3);
    twitter.post_tweet(3, 2);
    twitter.unfollow(1, 3);


    println!("News feed: {:?}", twitter.get_news_feed(1));
}

