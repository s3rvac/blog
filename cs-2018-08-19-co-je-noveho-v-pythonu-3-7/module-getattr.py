#!/usr/bin/env python3

import logging

def __getattr__(attr_name):
    logging.info(f'accessed {attr_name}')

    globals()[attr_name]
