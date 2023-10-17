import pandas as pd


def find_by_pid(item, pid='sports'):
    for i in item:
        if i['pid'] == pid:
            return i['value']
    return ''


# https://geek-docs.com/pandas/pandas-top-articles/1000100_python_pandas_index.html
def convert():
    data = pd.read_json('./db_dec_pem_pp.sales_unit_item.json')

    # 提取需要的数据
    data = data[['itemDocNo', 'attributeList']].rename(columns={'itemDocNo': 'model', 'attributeList': 'attr'})

    # 提取attributeList数组中的sports和ProductNature
    data['sports'] = data['attr'].map(lambda x: find_by_pid(x, 'sports'))
    data['nature'] = data['attr'].map(lambda x: find_by_pid(x, 'ProductNature'))
    data['catch_line'] = data['attr'].map(lambda x: find_by_pid(x, 'catchline'))

    data.reset_index(drop=True, inplace=True)

    # 选择需要的列
    data = (data[['model', 'catch_line', 'sports']]
            .astype({'model': 'str', 'catch_line': 'str', 'sports': 'object'}))

    # 重置索引
    data.reset_index(drop=True, inplace=True)

    # 展开sports数组
    data = data.set_index(['model', 'catch_line']).apply(pd.Series.explode).reset_index()

    # 提取sports中的sports_zh
    data['sports_zh'] = data['sports'].map(lambda x: x['sports_zh']).reset_index(drop=True)

    # 过滤sports_zh为空字符串的行
    data = data[data['sports_zh'] != '']

    # 选择需要的列
    # data = data[['model', 'sports_zh', 'catch_line']].reset_index(drop=True)
    data = data[['model', 'sports_zh']].reset_index(drop=True)

    print(data)
    data.to_csv('../runtime/dkt-sport-all.csv', index=False)
    data.to_excel('../runtime/dkt-sport-all.xlsx', index=False)

    data = data.drop_duplicates(subset=['sports_zh'], keep='first')

    print(data)
    data.to_csv('../runtime/dkt-distinct-sport-all.csv', index=False)
    data.to_excel('../runtime/dkt-distinct-sport-all.xlsx', index=False)


if __name__ == '__main__':
    convert()
