from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='py-msvc-demangler',
      author="Andrew V. Teylu",
      author_email="andrew@tey.lu",
      url='http://github.com/benfred/py-msvc-demangler/',
      description="A package for demangling MSVC C++ linker symbols",
      long_description=open("README.rst").read(),
      version="0.1.2",
      rust_extensions=[RustExtension('msvc_demangler', 'Cargo.toml',  binding=Binding.PyO3)],
      test_suite="tests",
      license="MIT",
      classifiers=[
        "Development Status :: 3 - Alpha",
        "Programming Language :: Python :: 3",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Topic :: Software Development :: Libraries",
        "Topic :: Utilities"],
      zip_safe=False)
