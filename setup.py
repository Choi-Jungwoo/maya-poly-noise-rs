from setuptools import setup
from setuptools_rust import RustExtension, Binding

setup(
    name="maya-poly-noise",
    version="0.1.0",
    packages=["maya_poly_noise"],
    rust_extensions=[
        RustExtension("maya_poly_noise.maya_poly_noise", binding=Binding.RustCPython)],
    zip_safe=False,
)
