# Maya PolyNoise
Using Rust In Maya Example
> ```Python API``` => ```Rust``` => ```Python API```

## Benchmark
- Python: 0.6s
- Rust: 0.04s
- C++: 0.04s

## Require
- Rust >= 1.49 (nightly)
- Maya >= 2016
- pip

## Install
First ```clone``` the code and then ```cd``` it into the code directory.
```shell
mayapy -m pip install -r requirements.txt && mayapy setup.py install
```

## Run!!!
```python
import maya.cmds as cmds
from maya_poly_noise import poly_noise

sphere = cmds.polySphere(radius=1, subdivisionsX=200, subdivisionsY=200)
poly_noise(sphere[0], 100)
```