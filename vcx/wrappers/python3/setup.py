from setuptools import setup, find_packages
from toml_utils import get_version_from_file
CARGO_FILE= '/home/mark/dev/sdk/vcx/libvcx/Cargo.toml'

setup(
    name='vcx',
    version=get_version_from_file(CARGO_FILE),
    description='Wrapper for libcxs',
    long_description='None...for now',
    author='Devin Fisher, Ryan Marsh, Mark Hadley, Doug Wightman',
    author_email='ryan.marsh@evernym.com',
    include_package_data=True,
    packages=find_packages(exclude=['demo', 'tests'])
)
