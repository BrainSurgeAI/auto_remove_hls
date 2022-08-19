from ast import Str, stmt

import hashlib
from tokenize import String
from typing import List

# class Stmt:
#     def __init__(self, id, room, date, url, thumbnail) -> None:
#         self.id = id
#         self.room = room
#         self.date = date
#         self.url = url
#         self.thumbnail = thumbnail


def read_file(filename) -> List:
    stmts = []
    with open(filename, 'r') as f:
        for line in f:
           stmts.append(create_stmt(line))
    f.close()

    return stmts

def create_stmt(line) -> str:
    strs = line.split("__")

    id = hashlib.md5(line.encode("utf-8")).hexdigest()
    room = ''
    timestamp = ''
    url = "https://www.example.com/hls/{}".format(line)
    thumbnail = "https://www.example.com/thumbnail/{}".format(line)

    for i, v in enumerate(strs):
        if i == 0:
            room = v[4:]
        elif i == 1:
            dates = v.split("-")
            year = dates[0]
            month = dates[1][0:2]
            day = dates[1][2:]
            hour = dates[2][0:2]
            minute = dates[2][2:4]
            second = dates[2][4:6]
            timestamp = "{}-{}-{} {}:{}:{}".format(year, month, day, hour, minute, second)
        else:
            continue
    
    stmt = "INSERT INTO video(id, room, date, url, thumbnail) VALUES('{}', '{}', TIMESTAMP '{}', '{}', '{}');".format(id, room, timestamp, url, thumbnail)
    print(stmt)
    return stmt
            
            
            



