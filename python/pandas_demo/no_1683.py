import pandas as pd


def invalid_tweets(tweets: pd.DataFrame) -> pd.DataFrame:
    return tweets[tweets['content'].str.len() > 15][['tweet_id']]


def main():
    data = [[1, 'Vote for Biden Biden Biden Biden'], [2, 'Let us make America great again!']]
    tweets = pd.DataFrame(data, columns=['tweet_id', 'content']).astype({'tweet_id': 'Int64', 'content': 'object'})
    print(invalid_tweets(tweets))


if __name__ == "__main__":
    main()
