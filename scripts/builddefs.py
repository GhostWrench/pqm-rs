import math
import json

from jinja2 import FileSystemLoader, Environment

# Read unit types from the schema
with open("./unitdb/schema.json", "r") as f:
    unitdb_schema = json.load(f)
# Read the unit and prefixes from the db.json file
with open("./unitdb/db.json", "r") as f:
    unitdb = json.load(f)

# function to evaluate numbers
def evalnumber(numstr):
    numstr = numstr.replace("pi", "math.pi")
    numstr = numstr.replace("^", "**")
    return "{:.14e}".format(eval(numstr))

# Arguments to put into the dictionary
template_kwargs = {}

# Get the dictionary defining the unit types
dimension_types = unitdb_schema["definitions"]["unit"]["properties"]["dimensions"]["properties"]
dimension_types = list(dimension_types.keys())
template_kwargs["num_dimension_types"] = len(dimension_types)
template_kwargs["dimension_types"] = dimension_types

# Get the prefixes from the unit database
dbprefixes = unitdb["prefixes"]
prefixes = []
for (symbol, data) in dbprefixes.items():
    prefixes.append({"symbol": symbol, "scale": evalnumber(data["scale"])})
template_kwargs["prefixes"] = prefixes

# Load the units from the unit database
dbunits = unitdb["units"]
units = []
for (symbol, data) in dbunits.items():
    if "offset" in data:
        continue
    scale = evalnumber(data["scale"])
    dimensions = ["0"]*len(dimension_types)
    for (name, dim) in data["dimensions"].items():
        dimensions[dimension_types.index(name)] = str(dim)
    symbols = [symbol]
    if "aliases" in data:
        symbols = symbols + data["aliases"]
    for s in symbols:
        units.append({
            "symbol": s,
            "scale": scale,
            "dims": ",".join(dimensions)
        })
template_kwargs["units"] = units

# Load the template and fill it in
loader = FileSystemLoader("src/templates")
env = Environment(loader=loader)
template = env.get_template("defs.rs")
code = template.render(**template_kwargs)
with open("./src/defs.rs", "w") as f:
    f.write(code)
