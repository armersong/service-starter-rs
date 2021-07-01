# coding: utf-8
import json
import hashlib
import requests
import time

def do_request(url, params={}, headers={}, body={}, method='POST', result_to_json=False, retCode=[200,] ):
    headers = headers or {}
    headers.update({'Content-Type': 'application/json'})
    resp = requests.request(method, url, params = params, headers = headers, data=json.dumps(body))
    if resp.status_code not in retCode:
        print('%d %s:%s' % (resp.status_code, resp.reason, resp.text))
        raise RuntimeError("%d: %s : %s" % (resp.status_code, resp.reason, resp.text))
    if result_to_json:
        if resp.text is None or resp.text == "":
            return {}
        return resp.json()
    return resp.text

def calc_sign(app_secret, params):
    keys = params.keys()
    keys.sort()
    src = ''
    for k in keys:
        src += k + params[k]
    src += app_secret
#    print("sign str: %s" % src)
    md5 = hashlib.md5()
    hash = md5.new(src)
    return hash.hexdigest().upper()

def time_it(f):
    def calc(*args, **kwargs):
        st = time.time()
        res = f(*args, **kwargs)
        et = time.time()
        print("spent %.3f seconds" % (et-st))
        return res
    return calc()
