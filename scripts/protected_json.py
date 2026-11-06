"""Bounded duplicate-preserving admission for protected engineering metadata.

This is not the product locale parser. It admits metadata with arrays/null/bools
under fixed engineering bounds; callers still check their own closed shape.
No identity/hash or origin substitutes for this representation boundary.
"""
import json

MAX_BYTES=1_000_000
MAX_DEPTH=8
MAX_TOKENS=65_536
MAX_ENCODED_STRING=6_144


def _pairs(items):
    value={}
    for key,child in items:
        if key in value:raise ValueError('duplicate decoded protected key')
        value[key]=child
    return value


def _integer(token):
    if not token or len(token)>10 or not token.isascii() or not token.isdecimal():raise ValueError('protected integer representation')
    value=int(token)
    if value>0xffffffff:raise ValueError('protected integer range')
    return value


def _reject(_token):
    raise ValueError('unselected protected primitive')


def read(raw):
    if type(raw)is not bytes or len(raw)>MAX_BYTES:raise ValueError('protected byte bound')
    text=raw.decode('utf-8','strict')
    if text.startswith('\ufeff'):raise ValueError('protected BOM')
    depth=0;tokens=0;inside=False;escaped=False;span=0
    for byte in raw:
        if inside:
            if byte==34 and not escaped:inside=False
            else:
                span+=1
                if span>MAX_ENCODED_STRING:raise ValueError('protected encoded string bound')
                escaped=not escaped and byte==92
            continue
        if byte==34:inside=True;escaped=False;span=0;tokens+=1
        elif byte in (123,91):
            depth+=1;tokens+=1
            if depth>MAX_DEPTH:raise ValueError('protected depth')
        elif byte in (125,93):
            depth-=1;tokens+=1
            if depth<0:raise ValueError('protected nesting')
        elif byte not in (32,9,10,13):tokens+=1
        if tokens>MAX_TOKENS:raise ValueError('protected token bound')
    if inside or depth:raise ValueError('protected complete document')
    value=json.loads(text,object_pairs_hook=_pairs,parse_int=_integer,parse_float=_reject,parse_constant=_reject)
    def checked(node):
        if type(node)is str:
            if len(node.encode('utf-8'))>4096 or any(not (0x20<=ord(ch)<=0x7e or ch in '·–—')for ch in node):raise ValueError('protected character/string profile')
        elif type(node)is list:
            for child in node:checked(child)
        elif type(node)is dict:
            for key,child in node.items():checked(key);checked(child)
    checked(value)
    return value
