import pandas as pd


# https://leetcode.cn/problems/big-countries/
def big_countries(world: pd.DataFrame) -> pd.DataFrame:
    # 过滤population大于25000000或者area大于3000000
    world = world[(world['population'] >= 25000000) | (world['area'] >= 3000000)]
    world = world[['name', 'population', 'area']]
    # 排序area列
    world = world.sort_values(by='area', ascending=False)
    # 重置索引
    world = world.reset_index(drop=True)

    return world


def test():
    data = [
        ['Afghanistan', 'Asia', 652230, 25500100, 20343000000],
        ['Albania', 'Europe', 28748, 2831741, 12960000000],
        ['Algeria', 'Africa', 2381741, 37100000, 188681000000],
        ['Andorra', 'Europe', 468, 78115, 3712000000],
        ['Angola', 'Africa', 1246700, 20609294, 100990000000]
    ]

    world = (pd.DataFrame(data, columns=['name', 'continent', 'area', 'population', 'gdp'])
             .astype({'name': 'object', 'continent': 'object', 'area': 'Int64', 'population': 'Int64', 'gdp': 'Int64'}))

    result = big_countries(world)
    result.to_excel('../runtime/big_countries.xlsx', index=False)
    result.to_csv('../runtime/big_countries.csv', index=False)

    print(result)


if __name__ == '__main__':
    test()
