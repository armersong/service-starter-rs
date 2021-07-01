# coding: utf-8
import unittest
from utils import do_request
from config import *

class AdminTestCase(unittest.TestCase):
    def login(self, user, pwd):
        body = {
            "account": user,
            "pass": pwd,
        }
        resp = do_request("%s/v1/admin/token" % URL_BASE, body=body, result_to_json=True)
        self.assertEqual(resp["result"], "0", "login failed!")
        print("token: %s" % resp["token"])
        return resp["token"]

    def test_login_ok(self):
        token = self.login(ADMIN_USER, ADMIN_PWD)
        self.assertNotEqual(token, "", "get token failed")

    def test_login_out_ok(self):
        token = self.login(ADMIN_USER, ADMIN_PWD)
        body = {
            "token": token,
        }
        resp = do_request("%s/v1/admin/token" % URL_BASE, method='DELETE', body=body, result_to_json=True)
        self.assertEqual(resp["result"], "0", "logout failed!")


