from ast import Str, stmt
import os
import hashlib
from tokenize import String
from typing import List

def read_file(filename) -> List:
    stmts = []
    for file in  os.listdir(filename):
        if file.endswith(".m3u8"):
            stmts.append(create_stmt(file))
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
    return "INSERT INTO video(id, room, date, url, thumbnail) VALUES('{}', '{}', TIMESTAMP '{}', '{}', '{}');".format(id, room, timestamp, url, thumbnail)
            
            
            



