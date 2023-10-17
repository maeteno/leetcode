import pandas as pd


def find_by_pid(item, pid='sports'):
    for i in item:
        if i['pid'] == pid:
            return i['value']


# https://geek-docs.com/pandas/pandas-top-articles/1000100_python_pandas_index.html
def convert():
    data = pd.read_json('./db_dec_pem_pp.sales_unit_item.json')

    # 提取需要的数据
    data = data[['itemDocNo', 'attributeList']].rename(columns={'itemDocNo': 'doc_no', 'attributeList': 'attr'})

    # 提取attributeList数组中的sports和ProductNature
    data['sports'] = data['attr'].map(lambda x: find_by_pid(x, 'sports'))
    data['name'] = data['attr'].map(lambda x: find_by_pid(x, 'ProductNature'))

    data.reset_index(drop=True, inplace=True)
    # 选择需要的列
    data = data[['doc_no', 'name', 'sports']]

    # 重置索引
    data.reset_index(drop=True, inplace=True)

    # 展开sports数组
    data = data.set_index(['doc_no', 'name']).apply(pd.Series.explode).reset_index()

    # 提取sports中的sports_zh
    data['sports_zh'] = data['sports'].map(lambda x: x['sports_zh']).reset_index(drop=True)

    # 过滤sports_zh为空字符串的行
    data = data[data['sports_zh'] != '']

    # 选择需要的列
    # data = data[['item_id', 'name', 'sports_zh']].reset_index(drop=True)
    data = data[['doc_no', 'sports_zh']].reset_index(drop=True)

    print(data)
    data.to_csv('../runtime/2023-item.csv', index=False)
    data.to_excel('../runtime/2023-item.xlsx', index=False)


if __name__ == '__main__':
    convert()
