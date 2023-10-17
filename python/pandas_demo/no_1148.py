import pandas as pd


# https://leetcode.cn/problems/article-views-i/

def article_views(views: pd.DataFrame) -> pd.DataFrame:
    df = views[views['author_id'] == views['viewer_id']]

    print(df)
    df.drop_duplicates(subset=['author_id'], inplace=True)
    df.sort_values(by=['author_id'], inplace=True)
    df.rename(columns={'author_id': 'id'}, inplace=True)

    df = df[['id']]

    return df


def test():
    data = [
        [1, 3, 5, '2019-08-01'],
        [1, 3, 6, '2019-08-02'],
        [2, 7, 7, '2019-08-01'],
        [2, 7, 6, '2019-08-02'],
        [4, 7, 1, '2019-07-22'],
        [3, 6, 4, '2019-07-21'],
        [5, 4, 4, '2019-07-21']
        [3, 4, 4, '2019-07-21']
        [3, 5, 4, '2019-07-21']
        [3, 4, 5, '2019-07-21']
    ]
    astype = {
        'article_id': 'Int64',
        'author_id': 'Int64',
        'viewer_id': 'Int64',
        'view_date': 'datetime64[ns]'
    }
    views = (pd.DataFrame(data, columns=['article_id', 'author_id', 'viewer_id', 'view_date'])
             .astype(astype))

    result = article_views(views)
    result.to_csv('../runtime/article_views.csv', index=False)
    print(result)


if __name__ == '__main__':
    test()
