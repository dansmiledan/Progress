
def clip(text: str, max_len:'int > 0'=80) -> str :
    pass

print(clip.__annotations__)


from inspect import signature
sig = signature(clip)
print('type sig.return_annotation')
print(type(sig.return_annotation))
print('sig.return_annotation: ')
print(sig.return_annotation)
print('sig.parameters:')
print(sig.parameters)