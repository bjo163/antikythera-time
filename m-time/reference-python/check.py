saros=2460409.263+6585.3223
def mabims(a,e): return a>=3.0 and e>=6.4
assert abs(saros-2466994.5853)<=1e-9
assert mabims(3.0,6.4)
assert not mabims(2.999,7.0)
assert not mabims(4.0,6.399)
print({"saros":saros,"mabimsBoundary":True})
