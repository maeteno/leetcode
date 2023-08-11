import pandas as pd


# https://leetcode.cn/problems/customers-who-never-order
def find_customers(customers: pd.DataFrame, orders: pd.DataFrame) -> pd.DataFrame:
    """
    Pandas 中：
        isin(values) 用于基于其值是否在给定集合 values 中来过滤和选择行。
        ~ 表示逻辑非。
    :param customers:  用户
    :param orders:  订单
    :return: 购买过的用户
    """
    # 选择 orders['customerId'] 中 'id' 不存在的行。
    df = customers[~customers['id'].isin(orders['customerId'])]

    # 创建一个只包含 name 列的数据框架
    # 并将列 name 重命名为 Customers。
    df = df[['name']].rename(columns={'name': 'Customers'})
    return df


def test():
    data = [[1, 'Joe'], [2, 'Henry'], [3, 'Sam'], [4, 'Max']]
    customers = pd.DataFrame(data, columns=['id', 'name']).astype({'id': 'Int64', 'name': 'object'})

    data = [[1, 3], [2, 1]]
    orders = pd.DataFrame(data, columns=['id', 'customerId']).astype({'id': 'Int64', 'customerId': 'Int64'})

    result = find_customers(customers, orders)

    result.to_csv('../runtime/customers.csv', index=False)
    print(result)


if __name__ == '__main__':
    test()
