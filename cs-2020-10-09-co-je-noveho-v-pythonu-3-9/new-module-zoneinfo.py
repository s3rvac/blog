#!/usr/bin/env python3
#
# Python 3.9: New module: zoneinfo
#

import datetime
import zoneinfo

dt = datetime.datetime(2020, 10, 10, tzinfo=zoneinfo.ZoneInfo('Europe/London'))
print(dt) # 2020-10-10 00:00:00+01:00
dt = datetime.datetime(2020, 10, 10, tzinfo=zoneinfo.ZoneInfo('Europe/Prague'))
print(dt) # 2020-10-10 00:00:00+02:00
