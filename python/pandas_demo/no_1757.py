import pandas as pd


# https://leetcode.cn/problems/recyclable-and-low-fat-products/
def find_products(products: pd.DataFrame) -> pd.DataFrame:
    products = products[(products['low_fats'] == 'Y') & (products['recyclable'] == 'Y')]
    return products[['product_id']]


def test():
    data = [
        ['0', 'Y', 'N'],
        ['1', 'Y', 'Y'],
        ['2', 'N', 'Y'],
        ['3', 'Y', 'Y'],
        ['4', 'N', 'N']
    ]

    products = (pd.DataFrame(data, columns=['product_id', 'low_fats', 'recyclable'])
                .astype({'product_id': 'int64', 'low_fats': 'category', 'recyclable': 'category'}))

    result = find_products(products)
    print(result)


test()
