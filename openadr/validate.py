# pip install xmlschema
import xmlschema
import os
import glob
import sys

if len(sys.argv) != 2:
    print("Missing argument path/to/xml")
    exit(0)

path = sys.argv[1]
file = open('openadr/xsd/pristine/oadr_20b.xsd').read()
schema = xmlschema.XMLSchema(file, base_url='openadr/xsd/pristine/')

for filename in glob.iglob(path + '**/*.xml', recursive=True):
    print("Validating file", filename)
    
    try:
        schema.validate(filename)
    except Exception as err:
        print(f"  Unexpected {err=}, \n  {type(err)=}")
