import json
import sys
import codecs
from glob import glob

jsondir = len(sys.argv) > 1 and sys.argv[1] or '.'


def TelList(strlis):
    strlis = strlis.strip()
    for rep in ('<em>', ''), ('</em>', ''), ('\t', ' '):
        strlis = strlis.replace(*rep)
    if not strlis:
        return ''
    r = []
    try:
        lis = json.loads(strlis)
    except json.JSONDecodeError as ex:
        print(ex, strlis)
        return ''
    for pair in reversed(sorted(lis, key=lambda x: (x['s']))):
        phone = pair['t'].strip()
        if phone:
            # r.append(phone+'/'+pair['s'])
            return phone
    return '，'.join(r)


def strip(val):
    val = val.strip()
    for rep in ('<em>', ''), ('</em>', ''), ('\t', ' '), (',', '，'):
        val = val.replace(*rep)
    return val


with codecs.open('brief.csv', 'w') as output:
    for f in glob(f'{jsondir}/*.json'):
        with codecs.open(f, encoding='utf-8') as openf:
            dct = json.load(openf)
            for res in dct['Result']:
                # for k in ['Address', 'Name', 'GW', 'OperName', 'RegistCapi', 'Email']:
                for k in ['Name', 'OperName', 'ContactNumber', 'Address', 'Email', 'GW', 'RegistCapi']:
                    val = strip(res[k]) or '_'
                    try:
                        print(val, end='\t', file=output)
                    except UnicodeEncodeError as ex:
                        print(ex, val, file=sys.stderr)
                        print('_', end='\t', file=output)
                print(TelList(res['TelList']), end='\t', file=output)
                print(file=output)
    #         "Name": "上海网鱼信息科技有限公司",
    #         "GW": "http://www.wywk.cn",
    #         "OperName": "董寅鸣",
    #         "RegistCapi": "4495.3013万元人民币",
    #         "TelList": "[{\"t\":\"021-64080700\",\"s\":\"2020\"},{\"t\":\"18917539806\",\"s\":\"2013\"},{\"t\":\"18917539807\",\"s\":\"2014\"}]",
    #         "X": 31.1023966793356,
    #         "Y": 121.327823380439
    #         "Email": "wuxhuhui@wywk.cn",
    #   "EmailList": [
    #     {
    #       "e": "jinhua@wywk.cn",
    #       "s": "2021"
    #     },
    #     {
    #       "e": "wuxuhui@wywk.cn",
    #       "s": "2018"
    #     }
    #   ],

    #   "Address": "上海市松江区新桥镇漕松路1号3幢301室",
    #   "Area": {
    #     "City": "上海市",
    #     "CityCode": "3101",
    #     "County": "松江区",
    #     "CountyCode": "310117",
    #     "Province": ""
    #   },
