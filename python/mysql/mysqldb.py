# -*- coding: utf-8 -*-

import MySQLdb
from getpass import getpass

if __name__ == '__main__':
    user = 'root'
    password = getpass('password of {} : '.format(user))
    host = 'localhost'
    database = 'sample'

    conn = MySQLdb.connect(
        user=user,
        passwd=password,
        host=host,
        database=database,
    )
    cur = conn.cursor()

    cur.execute('show tables;')
    print(cur.fetchall())
