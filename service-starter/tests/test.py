#!/usr/bin/env python

import unittest
import os
import sys

# setup module path
curPath = os.path.dirname( __file__)
SRC_ROOT = os.path.abspath(os.path.join( curPath, '..') )
if SRC_ROOT not in sys.path:
    sys.path.insert( 0, SRC_ROOT)

# select test file/unit
filter= "test_*.py"
if len(sys.argv) > 1 :
    if sys.argv[1] == '-h' or sys.argv[1] == '--help':
        print(
        '''usage: %s [test_name]
             by default, run all test( test_*.py)
             test_name not include test_
        ''' % sys.argv[0])
        sys.exit(1)
    filter = 'test_%s.py' % sys.argv[1]

# run testsuit
suite = unittest.TestLoader().discover( curPath, filter  )
unittest.TextTestRunner(verbosity = 2).run( suite )
