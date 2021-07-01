#!/bin/sh

HOST=127.0.0.1
PWD=123456
ysqldump -uroot -p$PWD -h$HOST admin --default-character-set=utf8 --triggers --routines --events --set-gtid-purged=OFF > admin.sql
