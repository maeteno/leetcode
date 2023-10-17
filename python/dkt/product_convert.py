import pandas as pd


# https://geek-docs.com/pandas/pandas-top-articles/1000100_python_pandas_index.html
def convert():
    data = pd.read_json('./db_dec_pem_pp.sales_unit_item.json')
    data = data[['itemDocNo', 'attributeList']].explode('attributeList')
    data = data[data["attributeList"].map(lambda x: x['pid'] == 'sports')]

    data = data.apply(lambda x: x.value, axis=1)
    print(data)
    data.to_excel('../runtime/2023-item.xlsx', index=False)


if __name__ == '__main__':
    convert()
