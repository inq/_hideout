#!/usr/bin/env mongo

conn = new Mongo()
db = conn.getDB('hideout_development')

res = db.dailyActivities.createIndex({date: 1}, {unique: true})
printjson(res)
