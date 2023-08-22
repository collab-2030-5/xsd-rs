import xmlschema
import os
import glob

file = open('openadr/xsd/oadr_20b.xsd').read()
schema = xmlschema.XMLSchema(file, base_url='openadr/xsd/')

for filename in glob.iglob('openadr/samplexml/' + '**/*.xml', recursive=True):
    print("Validating file", filename)
    schema.validate(filename)
